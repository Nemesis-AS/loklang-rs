pub mod api;
pub mod views;

use actix_web::web::{scope, ServiceConfig};

use api::register as register_api;
use views::register as register_views;

pub fn register(config: &mut ServiceConfig) {
    config.service(scope("/api/v1").configure(register_api));
    config.service(scope("").configure(register_views));
}
