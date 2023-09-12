use crate::metadata::{AudioMetadata, MetaPicture};
use crate::utils::{extract_arr_string, serialize_string_arr};

use actix_web::{error, web, Error};
use rusqlite::params;

use base64::Engine as _;

type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub enum SongActions {
    GetAllSongs,
    GetSongByID(String),
}

pub enum GetAllActions {
    Albums,
    Artists,
}

pub enum FilterActions {
    ByAlbum(String),
    ByArtist(String),
}

pub async fn create_tables(pool: &Pool) {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS songs(id TEXT NOT NULL PRIMARY KEY, filepath TEXT NOT NULL, title TEXT, artists TEXT, album TEXT, album_artists TEXT, year TEXT, genre TEXT, copyright TEXT, track_number TEXT, disc_number TEXT, track_total TEXT, disc_total TEXT, date TEXT, duration INT)",
        [],
    )
    .expect("An Error occurred while creating songs table!");

    conn.execute("CREATE TABLE IF NOT EXISTS pictures(image_id TEXT NOT NULL PRIMARY KEY, picture_type INT, mime TEXT, description TEXT, data BLOB, song_id TEXT, FOREIGN KEY(song_id) REFERENCES songs(id))", []).expect("An Error occurred while creating pictures table!");

    conn.execute("CREATE TABLE IF NOT EXISTS playlists(playlist_id TEXT PRIMARY KEY, title TEXT, songs TEXT)", []).expect("An Error occurred while creating playlists table!");
}

pub async fn add_songs(pool: &Pool, data: Vec<AudioMetadata>) -> Result<(), Error> {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    let mut stmt = conn
        .prepare("INSERT OR IGNORE INTO songs VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .unwrap();

    let mut stmt2 = conn
        .prepare("INSERT OR IGNORE INTO pictures VALUES(?, ?, ?, ?, ?, ?)")
        .unwrap();

    for item in data {
        stmt.execute(params![
            item.id.as_str(),
            item.filepath.as_str(),
            item.title.as_str(),
            serialize_string_arr(&item.artists).as_str(),
            item.album.as_str(),
            serialize_string_arr(&item.artists).as_str(),
            item.year.as_str(),
            serialize_string_arr(&item.genre).as_str(),
            item.copyright.as_str(),
            item.track_number.as_str(),
            item.disc_number.as_str(),
            item.track_total.as_str(),
            item.disc_total.as_str(),
            item.date.as_str(),
            item.duration
        ])
        .expect("An Error Occurred while inserting song to the DB!");

        for picture in item.pictures {
            stmt2
                .execute(params![
                    picture.id.as_str(),
                    picture.picture_type.clone(),
                    picture.mime.as_str(),
                    picture.description.as_str(),
                    picture.data.clone(),
                    item.id.as_str()
                ])
                .expect("An Error occurred while inserting image to the DB!");
        }
    }

    Ok(())
}

// ######################################### API ######################################################

pub async fn handle_song_action(
    pool: &Pool,
    action: SongActions,
) -> Result<Vec<AudioMetadata>, Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get()).await.unwrap().unwrap();

    web::block(move || match action {
        SongActions::GetAllSongs => get_all_songs(conn),
        SongActions::GetSongByID(song_id) => get_song_by_id(conn, song_id),
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

fn get_all_songs(conn: Connection) -> Result<Vec<AudioMetadata>, rusqlite::Error> {
    let stmt = conn.prepare("SELECT * FROM songs").unwrap();
    rows_to_metadata(&conn, stmt, [])
}

fn get_song_by_id(
    conn: Connection,
    song_id: String,
) -> Result<Vec<AudioMetadata>, rusqlite::Error> {
    let stmt = conn.prepare("SELECT * FROM songs WHERE id = ?").unwrap();
    rows_to_metadata(&conn, stmt, [song_id])
}

pub async fn handle_get_all_action(
    pool: &Pool,
    action: GetAllActions,
) -> Result<Vec<String>, Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get()).await.unwrap().unwrap();

    web::block(move || match action {
        GetAllActions::Albums => get_all_albums(conn),
        GetAllActions::Artists => get_all_artists(conn),
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

fn get_all_albums(conn: Connection) -> Result<Vec<String>, rusqlite::Error> {
    let stmt = conn.prepare("SELECT DISTINCT album FROM songs").unwrap();
    rows_to_string(stmt)
}

fn get_all_artists(conn: Connection) -> Result<Vec<String>, rusqlite::Error> {
    let stmt = conn.prepare("SELECT DISTINCT artists FROM songs").unwrap();
    rows_to_string(stmt)
}

pub async fn handle_filter_action(
    pool: &Pool,
    action: FilterActions,
) -> Result<Vec<AudioMetadata>, Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get()).await.unwrap().unwrap();

    web::block(move || match action {
        FilterActions::ByAlbum(album) => get_songs_by_album(conn, album),
        FilterActions::ByArtist(artist) => get_songs_by_artist(conn, artist),
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

fn get_songs_by_album(
    conn: Connection,
    album: String,
) -> Result<Vec<AudioMetadata>, rusqlite::Error> {
    let stmt = conn.prepare("SELECT * FROM songs WHERE album = ?").unwrap();
    rows_to_metadata(&conn, stmt, [album])
}

fn get_songs_by_artist(
    conn: Connection,
    artist: String,
) -> Result<Vec<AudioMetadata>, rusqlite::Error> {
    let stmt = conn
        .prepare("SELECT * FROM songs WHERE artists LIKE ?")
        .unwrap();
    rows_to_metadata(&conn, stmt, [format!("%{}%", artist)])
}

pub async fn get_picture_data_by_id(pool: &Pool, picture_id: String) -> Result<Vec<String>, Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get()).await.unwrap().unwrap();

    web::block(move || {
        let stmt = conn
            .prepare("SELECT mime, data FROM pictures WHERE image_id = ?")
            .unwrap();

        rows_to_picture_data(stmt, [picture_id])
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

fn rows_to_metadata(
    conn: &Connection,
    mut statement: rusqlite::Statement,
    params: impl rusqlite::Params,
) -> Result<Vec<AudioMetadata>, rusqlite::Error> {
    statement
        .query_map(params, |row| {
            let row_id: String = row.get(0).unwrap();
            let pics: Vec<MetaPicture> = get_pictures(conn, row_id).unwrap();

            Ok(AudioMetadata {
                id: row.get(0).unwrap(),
                filepath: row.get(1).unwrap(),
                title: row.get(2).unwrap(),
                artists: extract_arr_string(row.get(3).unwrap(), ", "),
                album: row.get(4).unwrap(),
                album_artists: extract_arr_string(row.get(5).unwrap(), ", "),
                year: row.get(6).unwrap(),
                genre: extract_arr_string(row.get(7).unwrap(), ", "),
                copyright: row.get(8).unwrap(),
                track_number: row.get(9).unwrap(),
                disc_number: row.get(10).unwrap(),
                track_total: row.get(11).unwrap(),
                disc_total: row.get(12).unwrap(),
                date: row.get(13).unwrap(),
                pictures: pics,
                duration: row.get(14).unwrap(),
            })
        })
        .and_then(Iterator::collect)
}

fn rows_to_string(mut statement: rusqlite::Statement) -> Result<Vec<String>, rusqlite::Error> {
    statement
        .query_map([], |row| Ok(row.get(0).unwrap()))
        .and_then(Iterator::collect)
}

fn rows_to_picture_data(
    mut statement: rusqlite::Statement,
    params: impl rusqlite::Params,
) -> Result<Vec<String>, rusqlite::Error> {
    statement
        .query_map(params, |row| {
            let mime: String = row.get(0).unwrap();
            let data: Vec<u8> = row.get(1).unwrap();
            let encoded: String = base64::engine::general_purpose::STANDARD_NO_PAD.encode(data);
            let final_str: String = format!("data:{};base64,{}", mime, encoded);

            Ok(final_str)
        })
        .and_then(Iterator::collect)
}

fn get_pictures(conn: &Connection, song_id: String) -> Result<Vec<MetaPicture>, rusqlite::Error> {
    conn.prepare("SELECT * FROM pictures WHERE song_id = ?")
        .unwrap()
        .query_map([song_id], |row| {
            Ok(MetaPicture {
                id: row.get(0).unwrap(),
                picture_type: row.get(1).unwrap(),
                mime: row.get(2).unwrap(),
                description: row.get(3).unwrap(),
                data: vec![], //row.get(4).unwrap(),
            })
        })
        .and_then(Iterator::collect)
}
