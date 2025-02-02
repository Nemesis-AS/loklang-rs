use crate::db::types::{DbError, DbPool};
use actix_web::{error, web, HttpResponse, Responder};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::models::Song;

async fn get_all_songs(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
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

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::resource("").route(web::get().to(get_all_songs)));
}
