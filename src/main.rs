#[macro_use]
extern crate diesel;

extern crate log;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
mod db;
mod errors;
mod post;
mod schema;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(post::init_routes))
        .bind("localhost:8000")?
        .run()
        .await
    // dotenv().ok();
    // env_logger::init();
    // let mut listenfd = ListenFd::from_env();
    // let mut server = HttpServer::new(|| App::new().service(index));

    // HttpServer::new(|| App::new().configure(post.init_routes))
    //     .bind("localhost:8000")?
    //     .run()
    //     .await
}
