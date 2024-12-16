use crate::utils::static_resolver::handle_embedded_file;
use actix_web::{
    get,
    web::{Path, ServiceConfig},
    HttpResponse, Responder,
};

// View Routes
#[get("/")]
pub async fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

#[get("/dist/{_:.*}")]
pub async fn dist(path: Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

pub fn register(config: &mut ServiceConfig) {
    config.service(index).service(dist);
}
