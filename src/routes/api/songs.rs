use actix_web::{error, web, HttpResponse, Responder, Result as AResult};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::models::{Song, SongRes};
use crate::db::types::{DbError, DbPool};
use crate::db::utils::{join_song_info, join_song_info_single};

async fn get_all_songs(pool: web::Data<DbPool>) -> AResult<impl Responder> {
    let songs = web::block(move || -> Result<Vec<SongRes>, DbError> {
        use crate::db::schema::songs::dsl::*;

        let mut conn = pool.get()?;

        let res: Vec<Song> = songs.select(Song::as_select()).load(&mut conn)?;

        let song_res: Vec<SongRes> = join_song_info(res, &mut conn).unwrap();

        Ok(song_res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(songs))
}

async fn get_song_by_id(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let song_id: String = path.into_inner();

    let song = web::block(move || -> Result<SongRes, DbError> {
        use crate::db::schema::songs::dsl::*;

        let mut conn = pool.get()?;

        let res: Song = songs
            .find(song_id)
            .select(Song::as_select())
            .first(&mut conn)?;

        let song_res: SongRes = join_song_info_single(res, &mut conn).unwrap();

        Ok(song_res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(song))
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::resource("").route(web::get().to(get_all_songs)));
    config.service(web::resource("/{song_id}").route(web::get().to(get_song_by_id)));
}
