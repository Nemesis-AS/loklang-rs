mod db;
mod metadata;
mod routes;
mod utils;

use actix_web::{web::Data, App, HttpServer};

use actix_cors::Cors;

use configparser::ini::Ini;

use r2d2::Pool;
use r2d2_sqlite::{self, SqliteConnectionManager};

use routes::register;

use std::path::PathBuf;
use std::{collections::HashMap, fs::create_dir_all};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "static/"]
pub struct Asset;

async fn setup_db(pool: &Pool<SqliteConnectionManager>, config: &HashMap<String, Option<String>>) {
    db::create_tables(pool).await;

    let def_path: Option<String> = Some(String::from("D:\\Music\\Test"));
    let config_path: String = config.get("rootdir").unwrap_or(&def_path).clone().unwrap();

    let path: PathBuf = PathBuf::from(config_path);
    let info: Vec<metadata::AudioMetadata> = utils::scan_dir(path);

    db::add_songs(pool, info).await.unwrap();
}

fn load_config() -> HashMap<String, Option<String>> {
    let config_path: PathBuf = PathBuf::from("./config.ini");

    if let Ok(_) = std::fs::metadata(&config_path) {
        let mut ini: Ini = Ini::new();
        let map: HashMap<String, HashMap<String, Option<String>>> = ini.load(config_path).unwrap();
        let config: &HashMap<String, Option<String>> = map.get("config").unwrap();

        if config.is_empty() {
            return HashMap::new();
        } else {
            return config.clone();
        }
    }

    // If the config file does not exist, create it with default values
    let mut ini: Ini = Ini::new();
    ini.set("config", "port", Some(String::from("8000")));
    ini.set("config", "rootdir", Some(String::from("D:\\Music\\Test")));
    ini.write(config_path).unwrap();

    HashMap::new()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    create_dir_all("./data")?;

    let manager = SqliteConnectionManager::file("data/db.sqlite");
    let pool = Pool::new(manager).unwrap();

    let config = load_config();
    setup_db(&pool, &config).await;

    let port: u16 = config
        .get("port")
        .unwrap_or(&Some(String::from("8000")))
        .clone()
        .unwrap()
        .parse()
        .unwrap();
    
    println!("Started server at PORT {}!", port);
    HttpServer::new(move || {
        let cors: Cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .configure(register)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
