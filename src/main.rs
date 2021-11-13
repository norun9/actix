#[macro_use]
extern crate diesel;

extern crate log;

use actix_web::{App, HttpServer};
use listenfd::ListenFd;

mod pkg;
mod post;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = pkg::prepare();
    let http = config.http;
    env_logger::init();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(post::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format_args!("{0}:{1}", http.host, http.port).to_string())?,
    };

    server.run().await
}
