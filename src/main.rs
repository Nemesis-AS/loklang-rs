mod db;
mod metadata;
mod routes;
mod utils;

use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};

use r2d2::Pool;
use r2d2_sqlite::{self, SqliteConnectionManager};

use routes::{get_song_by_id, get_songs};

use std::path::PathBuf;

async fn setup_db(pool: &Pool<SqliteConnectionManager>) {
    db::create_tables(pool).await;

    // @todo! Read the path from config;
    let path: PathBuf = PathBuf::from("D:\\Music\\Test");
    let info = utils::scan_dir(path);

    db::add_songs(pool, info).await.unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file("data/db.sqlite");
    let pool = Pool::new(manager).unwrap();

    setup_db(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(scope("/api/v1").service(get_songs).service(get_song_by_id))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
