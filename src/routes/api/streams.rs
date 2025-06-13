use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{error, web, Responder, Result as AResult};
use diesel::{QueryDsl, RunQueryDsl};

use crate::db::{
    models::Song,
    types::{DbError, DbPool},
};

async fn get_audio_stream(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> AResult<impl Responder> {
    let stream_id: String = path.into_inner();

    let song_data: Song = web::block(move || -> Result<Song, DbError> {
        use crate::db::schema::songs::dsl::*;

        let mut conn = pool.get()?;

        let res = songs.find(stream_id).first(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let path: PathBuf = PathBuf::from(&song_data.file_path);

    let audio_file = NamedFile::open_async(path).await.unwrap();

    Ok(audio_file)
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::resource("/{stream_id}").route(web::get().to(get_audio_stream)));
}
