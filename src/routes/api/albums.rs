use crate::db::{
    models::{Album, Song, SongRes},
    types::{DbError, DbPool}, utils::join_song_info,
};
use actix_web::{error, web, HttpResponse, Responder, Result as AResult};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

async fn get_all_albums(pool: web::Data<DbPool>) -> AResult<impl Responder> {
    let albums = web::block(move || -> Result<Vec<Album>, DbError> {
        use crate::db::schema::albums::dsl::*;

        let mut conn = pool.get()?;

        let res: Vec<Album> = albums.select(Album::as_select()).load(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(albums))
}

async fn get_album_by_id(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> AResult<impl Responder> {
    let album_id_input: String = path.into_inner();

    let album = web::block(move || -> Result<Album, DbError> {
        use crate::db::schema::albums::dsl::*;

        let mut conn = pool.get()?;

        let res = albums.find(album_id_input).first(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(album))
}

async fn get_songs_by_album(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> AResult<impl Responder> {
    let album_id_param: String = path.into_inner();

    let songs = web::block(move || -> Result<Vec<SongRes>, DbError> {
        use crate::db::schema::songs::dsl::*;

        let mut conn = pool.get()?;

        let res: Vec<Song> = songs
            .filter(album_id.eq(album_id_param))
            .select(Song::as_select())
            .load(&mut conn)?;

        let song_res: Vec<SongRes> = join_song_info(res, &mut conn).unwrap();

        Ok(song_res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(songs))
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::resource("").route(web::get().to(get_all_albums)));
    config.service(web::resource("/{album_id}").route(web::get().to(get_album_by_id)));
    config.service(web::resource("/{album_id}/songs").route(web::get().to(get_songs_by_album)));
}
