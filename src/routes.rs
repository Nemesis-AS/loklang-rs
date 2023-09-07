use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

use crate::db::{handle_song_action, SongActions};

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
// GET album/:id

// GET artists
// GET artist/:id

// GET playlists
// GET playlist/:id

// GET stream/:id
