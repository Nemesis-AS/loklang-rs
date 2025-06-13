use diesel::RunQueryDsl;

use crate::db::models::{
    AlbumArtist, AlbumInsertable, ArtistInsertable, SongArtist, SongInsertable,
};
use crate::db::types::DbPool;
use crate::metadata::AudioMetadata;

pub fn insert_metadata(pool: &DbPool, data: Vec<AudioMetadata>) {
    // use crate::db::schema::songs::dsl::*;

    // let songs_meta: Vec<SongInsertable> = data
    //     .into_iter()
    //     .map(|meta| SongInsertable {
    //         id: meta.id,
    //         title: meta.title,
    //         duration: Some(meta.duration as i32),
    //         file_path: meta.filepath,
    //         album_id: None,
    //         cover_image: None,
    //     })
    //     .collect();

    let mut conn = pool.get().unwrap();

    for item in data {
        let artists = item.artists;
        let album_title = item.album;

        // @todo! Case insensitive search in DB for artists
        // @todo! Update the database instead of doing nothing on collision
        let album_id_str: String = format!("{:x}", md5::compute(&album_title));
        let album: AlbumInsertable = AlbumInsertable {
            id: album_id_str.clone(),
            title: album_title.clone(),
            release_date: None,
            cover_image: None,
        };

        let insertable: SongInsertable = SongInsertable {
            id: item.id.clone(),
            title: item.title,
            duration: Some(item.duration as i32),
            file_path: item.filepath,
            album_id: Some(album_id_str.clone()),
            cover_image: None,
        };

        {
            use crate::db::schema::albums::dsl::*;

            diesel::insert_into(albums)
                .values(album)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("An error occurred while inserting album into db!");
        }

        {
            use crate::db::schema::songs::dsl::*;

            diesel::insert_into(songs)
                .values(&insertable)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("An error occurred while inserting songs!");
        }

        for artist in artists {
            let artist_id_str = format!("{:x}", md5::compute(&artist));
            let art: ArtistInsertable = ArtistInsertable {
                id: artist_id_str.clone(),
                name: artist.clone(),
                sort_name: artist.clone(),
            };

            {
                use crate::db::schema::artists::dsl::*;

                diesel::insert_into(artists)
                    .values(art)
                    .on_conflict_do_nothing()
                    .execute(&mut conn)
                    .expect("An error occurred while inserting artist into db!");
            }

            {
                use crate::db::schema::song_artists::dsl::*;

                let song_artist: SongArtist = SongArtist {
                    song_id: item.id.clone(),
                    artist_id: artist_id_str.clone(),
                    role: None,
                };

                diesel::insert_into(song_artists)
                    .values(&song_artist)
                    .on_conflict_do_nothing()
                    .execute(&mut conn)
                    .expect("An error occurred while linking artist to song relationship");
            }

            {
                use crate::db::schema::album_artists::dsl::*;

                let album_artist: AlbumArtist = AlbumArtist {
                    album_id: album_id_str.clone(),
                    artist_id: artist_id_str,
                };

                diesel::insert_into(album_artists)
                    .values(&album_artist)
                    .on_conflict_do_nothing()
                    .execute(&mut conn)
                    .expect("An error occurred while linking artist to album");
            }
        }

        // ORDER: ALbum, song, artist, relationships

        // {
        //     use crate::db::schema::
        // }
    }
}
