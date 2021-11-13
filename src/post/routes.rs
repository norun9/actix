use crate::pkg;
use crate::post::model::Post;

use actix_web::{get, web, HttpResponse};

#[get("/posts")]
async fn index() -> Result<HttpResponse, pkg::InternalError> {
    let posts = Post::index()?;
    Ok(HttpResponse::Ok().json(posts))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
