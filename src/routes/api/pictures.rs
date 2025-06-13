// let picture_id: String = data.into_inner();
//     let res: Vec<String> = get_picture_data_by_id(&db, picture_id).await.unwrap();

//     if res.is_empty() {
//         HttpResponse::NotFound().body("Not Found!")
//     } else {
//         HttpResponse::Ok().body(res[0].clone())
//     }

use actix_web::{web, HttpResponse, Responder, Result as AResult};
// use serde_json::json;

// use crate::db::types::DbPool;

async fn get_picture(/* path: web::Path<String>, pool: web::Data<DbPool> */) -> AResult<impl Responder> {
    // let asset_id: String = path.into_inner();

    // let song = 
    let img: String = String::from("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACAAQMAAAD58POIAAAAAXNSR0IArs4c6QAAAAZQTFRFT2eBepOuF/rBxQAAAUxJREFUSMft1D1qwzAUB/AXNHiLLqDah+hiqMBXcuiiocQKgnbr3CEH6agkB+gVXukF3uglUWWcONJLoEM7dMh/0w8h3gcI4Ja/CUoLRXrGBoscOmLQcyCTwgw/sZYp+AjzHLxmsOXwoVIQEdZlCjaCYqAYeJ1B2HtdBZygCHvUDYM6haE7kw0IsO8ZdMSgQQZVPuRb/mWEb6VPty+wbYhB12dArXmCFlIglcF7iyWHeQZvCwbrH+F14SUDzOGZ33jhb1yAo0vIS18RcTCaQddnYKkhBtKn8PtUwS8By1K4wwk2wWJZCRf8EXbBjzDuToHYYQTnHB2hsCPYCSC2f7dz1kzQjlAfQQ6gLuDLWX0dqsMcTITHM0gO4417/sbDFajTwlSVApEaYCrdDiDPzYkNGh1hNUHwXR9hGtAs2IZUcRqhAliCxAiH20/xDU7I5BRNJ7lxAAAAAElFTkSuQmCC");

    Ok(HttpResponse::Ok().body(img))
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::resource("/{asset_id}").route(web::get().to(get_picture)));
}
