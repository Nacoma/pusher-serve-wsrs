mod messages;

use std::sync::{Arc, Mutex};
use actix::Addr;
use actix_web::{HttpRequest, Responder, web, post, get};
use serde_json::json;
use crate::api::messages::Event;
use crate::{AppRepo, HttpResponse, WebSocketHandler};
use crate::ws::Broadcast;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/apps/{app}/events")]
pub async fn events(
    req: HttpRequest,
    event: web::Json<Event>,
    handler: web::Data<Addr<WebSocketHandler>>,
    repo: web::Data<Arc<Mutex<dyn AppRepo>>>,
) -> impl Responder {
    println!("{:?}", event);

    let app_id: String = req.match_info().get("app").unwrap().parse().unwrap();

    let app = repo.lock()
        .unwrap()
        .find_by_id(&app_id)
        .unwrap()
        .clone();

    let channels = if let Some(channels) = &event.channels {
        channels.clone()
    } else if let Some(channel) = &event.channel {
        vec![channel.clone()]
    } else {
        vec![]
    };

    let broadcast = Broadcast {
        channels,
        event: event.name.clone(),
        except: None,
        message: event.data.clone(),
        app,
    };

    handler.do_send(broadcast);

    HttpResponse::Ok()
        .header("content-type", "application/json")
        .json(json!({}))
}
