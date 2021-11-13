use crate::pkg;
use crate::post::model;
use actix_web::{get, post, put, web, HttpResponse};

type PostResult = Result<HttpResponse, pkg::InternalError>;

#[get("/posts")]
async fn index() -> PostResult {
    let posts = model::Post::index()?;
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/{id}")]
async fn find(id: web::Path<i32>) -> PostResult {
    let post = model::Post::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

#[post("/posts")]
async fn create(new_post: web::Json<model::InsertPost>) -> PostResult {
    let result = model::Post::create(new_post.into_inner());
    Ok(HttpResponse::Ok().json(result.unwrap()))
}

#[put("/posts")]
async fn update(target: web::Json<model::UpdatePost>) -> PostResult {
    let result = model::Post::update(target.into_inner());
    Ok(HttpResponse::Ok().json(result.unwrap()))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
}
