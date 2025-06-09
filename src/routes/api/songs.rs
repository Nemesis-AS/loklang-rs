use crate::db::types::{DbError, DbPool};
use actix_web::{error, web, HttpResponse, Responder, Result as AResult};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::models::Song;

async fn get_all_songs(pool: web::Data<DbPool>) -> AResult<impl Responder> {
    let songs = web::block(move || -> Result<Vec<Song>, DbError> {
        use crate::db::schema::songs::dsl::*;

        let mut conn = pool.get()?;

        let res = songs.select(Song::as_select()).load(&mut conn)?;

        Ok(res)
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

    let song = web::block(move || -> Result<Song, DbError> {
        use crate::db::schema::songs::dsl::*;

        let mut conn = pool.get()?;

        let res: Song = songs
            .find(song_id)
            .select(Song::as_select())
            .first(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(song))
}

// async fn get_songs_by_album(
//     path: web::Path<String>,
//     pool: web::Data<DbPool>,
// ) -> AResult<impl Responder> {
//     let album_id: String = path.into_inner();

//     let songs = web::block(move || -> Result<Vec<Song>, DbError> {
//         use crate::db::schema::songs::dsl::*;

//         let mut conn = pool.get()?;

//         let res: Vec<Song> = songs.filter().select(Song::as_select()).load(&mut conn)?;

//         Ok(res)
//     })
//     .await?
//     .map_err(error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(songs))
// }

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::resource("").route(web::get().to(get_all_songs)));
    config.service(web::resource("/{song_id}").route(web::get().to(get_song_by_id)));
}
