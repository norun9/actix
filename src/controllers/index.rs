use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
