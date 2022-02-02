use std::sync::Arc;

use actix::Addr;
use actix_web::{post, web, HttpRequest, Responder};
use serde_json::json;

use crate::ws::Broadcast;
use crate::{AppRepo, HttpResponse, WebSocketHandler};
use parking_lot::Mutex as PMutex;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppQuery {
    pub app_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: String,
    #[serde(with = "serde_with::json::nested")]
    pub data: serde_json::Value,
    pub channels: Option<Vec<String>>,
    pub channel: Option<String>,
    pub socket_id: Option<String>,
}

#[post("/apps/{app_id}/events")]
pub async fn publish(
    _req: HttpRequest,
    query: web::Path<AppQuery>,
    event: web::Json<Event>,
    handler: web::Data<Addr<WebSocketHandler>>,
    repo: web::Data<Arc<PMutex<dyn AppRepo>>>,
) -> impl Responder {
    let app = repo.lock().find_by_id(query.app_id).unwrap();

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
