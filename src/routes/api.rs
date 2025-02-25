use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{
    get,
    web::{Data, Path},
    HttpRequest, HttpResponse, Responder,
};

use crate::{
    db::{
        get_picture_data_by_id, handle_filter_action, handle_get_all_action, handle_song_action,
        FilterActions, GetAllActions, SongActions,
    },
    metadata::AudioMetadata,
};

type DataDB = Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>;

#[get("/songs")]
pub async fn get_songs(db: DataDB) -> HttpResponse {
    let res = handle_song_action(&db, SongActions::GetAllSongs)
        .await
        .unwrap();
    HttpResponse::Ok().json(res)
}

// GET song/:id
#[get("/songs/{id}")]
pub async fn get_song_by_id(data: Path<String>, db: DataDB) -> HttpResponse {
    let song_id = data.into_inner();
    // println!("SongID: {:?}", song_id);
    let res = handle_song_action(&db, SongActions::GetSongByID(song_id))
        .await
        .unwrap();
    HttpResponse::Ok().json(res)
}

// GET albums
#[get("/albums")]
pub async fn get_albums(db: DataDB) -> HttpResponse {
    let res = handle_get_all_action(&db, GetAllActions::Albums)
        .await
        .unwrap();
    HttpResponse::Ok().json(res)
}
// GET album/:id
#[get("/albums/{id}")]
pub async fn get_songs_by_album(data: Path<String>, db: DataDB) -> HttpResponse {
    let album: String = data.into_inner();
    // println!("Album: {:?}", album);
    let res = handle_filter_action(&db, FilterActions::ByAlbum(album))
        .await
        .unwrap();
    HttpResponse::Ok().json(res)
}

// GET artists
#[get("/artists")]
pub async fn get_artists(db: DataDB) -> HttpResponse {
    let res = handle_get_all_action(&db, GetAllActions::Artists)
        .await
        .unwrap();
    HttpResponse::Ok().json(res)
}
// GET artist/:id
#[get("/artists/{id}")]
pub async fn get_songs_by_artist(data: Path<String>, db: DataDB) -> HttpResponse {
    let artist: String = data.into_inner();
    // println!("Artist: {:?}", artist);
    let res = handle_filter_action(&db, FilterActions::ByArtist(artist))
        .await
        .unwrap();
    HttpResponse::Ok().json(res)
}

// GET playlists
// GET playlist/:id

// GET stream/:id
#[get("/stream/{id}")]
pub async fn get_stream_by_id(req: HttpRequest, data: Path<String>, db: DataDB) -> impl Responder {
    let song_id: String = data.into_inner();
    let res: Vec<AudioMetadata> = handle_song_action(&db, SongActions::GetSongByID(song_id))
        .await
        .unwrap();
    if res.is_empty() {
        HttpResponse::NotFound().body("Not Found!")
    } else {
        let song: &AudioMetadata = &res[0];
        let path: PathBuf = PathBuf::from(song.filepath.clone());

        let named_file = NamedFile::open_async(path).await.unwrap();
        named_file.into_response(&req)
    }
}

// GET picture/:id
#[get("/picture/{id}")]
pub async fn get_picture(data: Path<String>, db: DataDB) -> HttpResponse {
    let picture_id: String = data.into_inner();
    let res: Vec<String> = get_picture_data_by_id(&db, picture_id).await.unwrap();

    if res.is_empty() {
        HttpResponse::NotFound().body("Not Found!")
    } else {
        HttpResponse::Ok().body(res[0].clone())
    }
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config
        .service(get_songs)
        .service(get_song_by_id)
        .service(get_albums)
        .service(get_songs_by_album)
        .service(get_artists)
        .service(get_songs_by_artist)
        .service(get_stream_by_id)
        .service(get_picture);
}
