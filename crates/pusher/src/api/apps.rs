use std::sync::{Arc};

use actix_web::{get, post, web, HttpRequest, Responder};
use parking_lot::Mutex;
use serde::Deserialize;

use crate::{AppRepo, HttpResponse, PusherApp};

#[get("/apps")]
pub async fn all(_req: HttpRequest, repo: web::Data<Arc<Mutex<dyn AppRepo>>>) -> impl Responder {
    let apps = repo.lock().all();

    HttpResponse::Ok().json(apps)
}

#[derive(Deserialize)]
pub struct CreateAppPayload {
    name: String,
}

#[post("/apps")]
pub async fn create(
    _req: HttpRequest,
    body: web::Json<CreateAppPayload>,
    repo: web::Data<Arc<Mutex<dyn AppRepo>>>,
) -> impl Responder {
    let app = PusherApp::new(body.name.clone());

    repo.lock().insert_app(&app).unwrap();

    HttpResponse::Created().json(app)
}

#[derive(Deserialize)]
pub struct AppQuery {
    app_id: i64,
}
