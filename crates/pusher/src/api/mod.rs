mod errors;
pub mod apps;
pub mod channels;
pub mod events;

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
}
