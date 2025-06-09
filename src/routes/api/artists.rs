use actix_web::{error, web, HttpResponse, Responder, Result as AResult};
use diesel::{RunQueryDsl, SelectableHelper, ExpressionMethods, QueryDsl};

use crate::db::models::Artist;
use crate::db::types::{DbError, DbPool};

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

async fn get_artist_by_id(path: web::Path<String>, pool: web::Data<DbPool>) -> AResult<impl Responder> {
    let artist_id_param: String = path.into_inner();
    
    let artist = web::block(move || -> Result<Artist, DbError> {
        use crate::db::schema::artists::dsl::*;

        let mut conn = pool.get()?;

        let res = artists.filter(id.eq(artist_id_param)).select(Artist::as_select()).first(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(artist))
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::resource("").route(web::get().to(get_all_artists)));
    config.service(web::resource("/{artist_id}").route(web::get().to(get_artist_by_id)));
}
