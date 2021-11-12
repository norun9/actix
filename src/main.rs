#[macro_use]
extern crate diesel;

extern crate log;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
// use listenfd::ListenFd;
// use std::env;
mod config;
mod db;
mod errors;
mod post;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let http = match envy::from_env::<config::HTTP>() {
        Ok(val) => val,
        Err(error) => panic!("{:#?}", error),
    };
    HttpServer::new(|| App::new().configure(post::init_routes))
        .bind(format_args!("{0}:{1}", http.host, http.port).to_string())?
        .run()
        .await
    // env_logger::init();
    // let mut listenfd = ListenFd::from_env();
    // let mut server = HttpServer::new(|| App::new().service(index));

    // HttpServer::new(|| App::new().configure(post.init_routes))
    //     .bind("localhost:8000")?
    //     .run()
    //     .await
}
