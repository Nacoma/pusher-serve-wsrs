pub mod apps;
mod errors;
pub mod events;

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
}
