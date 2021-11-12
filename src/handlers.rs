use actix_web::Responder;

pub async fn get_posts() -> impl Responder {
    format!("hello from get posts")
}
