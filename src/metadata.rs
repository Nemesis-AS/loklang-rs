use crate::utils::{extract_arr, extract_arr_string};

use lofty::flac::{FlacFile, FlacProperties};
use lofty::id3::v2::FrameValue;
use lofty::mpeg::MpegFile;
use lofty::ogg::{OggPictureStorage, VorbisComments};
use lofty::{AudioFile, ParseOptions};

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AudioMetadata {
    pub id: String,
    pub filepath: String,

    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub album_artists: Vec<String>,
    pub year: String,
    pub genre: Vec<String>,
    pub copyright: String,
    pub track_number: String,
    pub disc_number: String,
    pub track_total: String,
    pub disc_total: String,
    pub date: String,
    pub pictures: Vec<MetaPicture>,
    pub duration: u64,
}

#[derive(Default, Serialize, Deserialize)]
pub struct MetaPicture {
    pub id: String,
    pub picture_type: u8,
    pub mime: String,
    pub description: String,
    pub data: Vec<u8>,
}

impl std::fmt::Debug for MetaPicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaPicture")
            .field("picture_type", &self.picture_type)
            .field("mime", &self.mime)
            .field("description", &self.description)
            // .field("data", "Vec<u8>")
            .finish()
    }
}

pub fn get_metadata(filepath: PathBuf) -> Option<AudioMetadata> {
    let format: &str = filepath
        .extension()
        .expect("Invalid File!")
        .to_str()
        .unwrap_or("unknown");

    let meta: Option<AudioMetadata> = match format {
        "flac" => read_flac(filepath),
        "mp3" => read_mp3(filepath),
        // "ogg" => println!("OGG"),
        // "wav" => println!("WAV"),
        _ => {
            // println!("Unknown Format!");
            None
        }
    };

    meta
}

fn read_flac(filepath: PathBuf) -> Option<AudioMetadata> {
    let mut content: File = File::open(&filepath).unwrap();

    if let Ok(file) = FlacFile::read_from(&mut content, ParseOptions::new()) {
        let comments: &VorbisComments = file.vorbis_comments()?;

        let mut meta: AudioMetadata = AudioMetadata {
            filepath: String::from(filepath.to_str().unwrap_or("")),
            id: format!("{:x}", md5::compute(filepath.to_str().unwrap_or(""))),
            ..Default::default()
        };

        for comment in comments.items() {
            let (key, value) = comment;

            match key {
                "TITLE" => meta.title = value.to_string(),
                "ARTIST" => meta.artists = extract_arr(value, ", "),
                "ALBUM" => meta.album = value.to_string(),
                "ALBUMARTIST" => meta.album_artists = extract_arr(value, ", "),
                "YEAR" => meta.year = value.to_string(),
                "GENRE" => meta.genre = extract_arr(value, ", "),
                "COPYRIGHT" => meta.copyright = value.to_string(),
                "TRACKNUMBER" => meta.track_number = value.to_string(),
                "DISCNUMBER" => meta.disc_number = value.to_string(),
                "TRACKTOTAL" => meta.track_total = value.to_string(),
                "DISCTOTAL" => meta.disc_total = value.to_string(),
                "DATE" => meta.date = value.to_string(),
                _ => (),
            };
        }

        let pictures: &[(lofty::Picture, lofty::PictureInformation)] = file.pictures();
        for item in pictures {
            let (pic, _) = item;

            let mut pic_id = meta.filepath.clone();
            pic_id.push_str(pic.pic_type().as_u8().to_string().as_str());

            let res: MetaPicture = MetaPicture {
                id: format!("{:x}", md5::compute(pic_id)),
                picture_type: pic.pic_type().as_u8(),
                mime: pic.mime_type().as_str().to_string(),
                description: String::from(pic.description().unwrap_or("")),
                data: pic.data().to_vec(),
            };

            meta.pictures.push(res);
        }

        let props: &FlacProperties = file.properties();
        meta.duration = props.duration().as_secs();

        Some(meta)
    } else {
        eprintln!(
            "Could not parse file metadata!\n{}",
            &filepath.to_str().unwrap()
        );
        None
    }
}

fn read_mp3(filepath: PathBuf) -> Option<AudioMetadata> {
    let mut content: File = File::open(&filepath).unwrap();

    if let Ok(file) = MpegFile::read_from(&mut content, ParseOptions::new()) {
        let mut meta: AudioMetadata = AudioMetadata {
            filepath: String::from(filepath.to_str().unwrap_or("")),
            id: format!("{:x}", md5::compute(filepath.to_str().unwrap_or(""))),
            ..Default::default()
        };

        // ID3v1
        if let Some(v1) = file.id3v1() {
            if let Some(title) = &v1.title {
                meta.title = title.clone();
            }

            if let Some(artist) = &v1.artist {
                meta.artists = extract_arr(artist, ", ");
            }

            if let Some(album) = &v1.album {
                meta.album = album.clone();
            }

            if let Some(year) = &v1.year {
                meta.year = year.clone();
            }

            if let Some(track_number) = &v1.track_number {
                meta.track_number = track_number.to_string();
            }

            if let Some(genre) = &v1.genre {
                meta.genre = vec![genre.to_string()];
            }
        }

        //ID3v2
        if let Some(v2) = file.id3v2() {
            for frame in v2 {
                // println!(
                //     "Title: {:?}\nValue: {:?}\n\n",
                //     frame.id_str(),
                //     frame.content()
                // );

                // @todo()! Missing Genre, Track Position, Duration;
                match frame.id_str() {
                    "TIT1" | "TIT2" => meta.title = decode_text_frame(frame.content()),
                    "TPE1" | "TPE2" => {
                        let artists = extract_arr_string(decode_text_frame(frame.content()), ", ");
                        meta.artists = artists.clone();
                        meta.album_artists = artists;
                    }
                    "TALB" => meta.album = decode_text_frame(frame.content()),
                    "TYER" | "TDOR" => meta.year = decode_text_frame(frame.content()),
                    "TCOP" => meta.copyright = decode_text_frame(frame.content()),
                    "TDAT" => meta.date = decode_text_frame(frame.content()),
                    "APIC" => meta.pictures = decode_picture_frame(frame.content(), &meta.filepath),
                    // "TLEN" => meta.duration = decode_text_frame(frame.content()),
                    _ => (),
                };
            }
        }

        Some(meta)
    } else {
        eprintln!(
            "Could not parse file metadata!\n{}",
            &filepath.to_str().unwrap()
        );
        None
    }
}

// Utils
fn decode_text_frame(frame: &FrameValue) -> String {
    match frame {
        FrameValue::Text(tif) => tif.value.clone(),
        FrameValue::UserText(etf) => etf.content.clone(),
        _ => String::new(),
    }
}

fn decode_picture_frame(frame: &FrameValue, filepath: &str) -> Vec<MetaPicture> {
    if let FrameValue::Picture(img) = frame {
        let pic: &lofty::Picture = &img.picture;
        let mut pic_id: String = filepath.to_string();
        pic_id.push_str(pic.pic_type().as_u8().to_string().as_str());

        vec![MetaPicture {
            id: format!("{:x}", md5::compute(pic_id)),
            picture_type: pic.pic_type().as_u8(),
            mime: pic.mime_type().as_str().to_string(),
            description: String::from(pic.description().unwrap_or("")),
            data: pic.data().to_vec(),
        }]
    } else {
        vec![]
    }
}
