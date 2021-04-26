use actix_web::{HttpRequest, web, Responder, HttpResponse};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use crate::pusher::Pusher;


#[derive(Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
}
pub async fn store(
    _req: HttpRequest,
    info: web::Json<CreateAppRequest>,
    pusher: web::Data<Arc<Mutex<Pusher>>>
) -> impl Responder {
    let res = pusher
        .lock()
        .unwrap()
        .create_app(info.name.clone())
        .expect(":(");

    HttpResponse::Ok().json(&res)
}

pub async fn index(
    _req: HttpRequest,
    pusher: web::Data<Arc<Mutex<Pusher>>>,
) -> impl Responder {
    let res = pusher
        .lock()
        .unwrap()
        .list_apps()
        .expect(":(");

    HttpResponse::Ok().json(res)
}

pub async fn delete(
    req: HttpRequest,
    pusher: web::Data<Arc<Mutex<Pusher>>>,
) -> impl Responder {
    let app_id: String = req.match_info().get("app").unwrap().parse().unwrap();

    pusher
        .lock()
        .unwrap()
        .delete_app(app_id.parse::<i32>().unwrap())
        .expect(":(");

    HttpResponse::Ok().json(serde_json::json!({
        "message": "OK"
    }))
}
