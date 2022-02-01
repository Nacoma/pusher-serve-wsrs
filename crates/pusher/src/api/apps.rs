use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, HttpRequest, Responder};
use serde::Deserialize;

use crate::{AppRepo, HttpResponse, PusherApp};

#[get("/apps")]
pub async fn all(_req: HttpRequest, repo: web::Data<Arc<Mutex<dyn AppRepo>>>) -> impl Responder {
    let apps = repo.lock().unwrap().all();

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

    repo.lock().unwrap().insert_app(&app);

    HttpResponse::Created().json(app)
}

#[derive(Deserialize)]
pub struct AppQuery {
    app_id: i64,
}
