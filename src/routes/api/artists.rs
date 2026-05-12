use actix_web::{error, web, HttpResponse, Responder, Result as AResult};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::models::{Artist, Song, SongRes};
use crate::db::schema::{song_artists, songs};
use crate::db::types::{DbError, DbPool};
use crate::db::utils::join_song_info;
// use diesel::associations::HasTable

async fn get_all_artists(pool: web::Data<DbPool>) -> AResult<impl Responder> {
    let artists = web::block(move || -> Result<Vec<Artist>, DbError> {
        use crate::db::schema::artists::dsl::*;

        let mut conn = pool.get()?;

        let res = artists.select(Artist::as_select()).load(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(artists))
}

async fn get_artist_by_id(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> AResult<impl Responder> {
    let artist_id_param: String = path.into_inner();

    let artist = web::block(move || -> Result<Artist, DbError> {
        use crate::db::schema::artists::dsl::*;

        let mut conn = pool.get()?;

        let res = artists
            .filter(id.eq(artist_id_param))
            .select(Artist::as_select())
            .first(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(artist))
}

async fn get_songs_by_artist(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> AResult<impl Responder> {
    let artist_id_param: String = path.into_inner();

    let artist_songs = web::block(move || -> Result<Vec<SongRes>, DbError> {
        // use crate::db::schema::songs::dsl::*;

        let mut conn = pool.get()?;

        let res: Vec<Song> = song_artists::table
            .filter(song_artists::artist_id.eq(&artist_id_param))
            .inner_join(songs::table.on(song_artists::song_id.eq(songs::id)))
            .select(Song::as_select())
            .load(&mut conn)?;

        let song_res: Vec<SongRes> = join_song_info(res, &mut conn).unwrap();

        Ok(song_res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(artist_songs))
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::resource("").route(web::get().to(get_all_artists)));
    config.service(web::resource("/{artist_id}").route(web::get().to(get_artist_by_id)));
    config.service(web::resource("/{artist_id}/songs").route(web::get().to(get_songs_by_artist)));
}
