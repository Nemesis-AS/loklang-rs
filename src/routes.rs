use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

use crate::db::{
    handle_filter_action, handle_get_all_action, handle_song_action, FilterActions, GetAllActions,
    SongActions,
};

type DataDB = Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>;

// GET songs
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
    println!("SongID: {:?}", song_id);
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
    println!("Album: {:?}", album);
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
    println!("Artist: {:?}", artist);
    let res = handle_filter_action(&db, FilterActions::ByArtist(artist))
        .await
        .unwrap();
    HttpResponse::Ok().json(res)
}

// GET playlists
// GET playlist/:id

// GET stream/:id
