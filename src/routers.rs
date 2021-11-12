use crate::controllers::index;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
}
