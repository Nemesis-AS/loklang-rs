use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Selectable, Queryable, Identifiable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Song {
    pub id: String,
    pub title: String,
    pub file_path: String,
    pub duration: Option<i32>,
    pub album_id: Option<String>,
    pub cover_image: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Selectable, Queryable, Identifiable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub sort_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Selectable, Queryable, Identifiable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Album {
    pub id: String,
    pub title: String,
    pub release_date: Option<String>,
    pub cover_image: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Selectable, Queryable, Identifiable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::playlists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Selectable, Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::song_artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SongArtist {
    pub song_id: String,
    pub artist_id: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Selectable, Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::album_artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlbumArtist {
    pub album_id: String,
    pub artist_id: String,
}

#[derive(Debug, Clone, Selectable, Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::playlist_songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlaylistSong {
    pub playlist_id: String,
    pub song_id: String,
    pub position: i32,
}
