use actix_web::{App, HttpServer};
mod routers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routers::routes))
        .bind("localhost:8000")?
        .run()
        .await
}
