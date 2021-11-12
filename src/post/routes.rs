use crate::errors::InternalError;
use crate::post::model::Post;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/posts")]
async fn index() -> Result<HttpResponse, InternalError> {
    let posts = Post::index()?;
    Ok(HttpResponse::Ok().json(posts))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
