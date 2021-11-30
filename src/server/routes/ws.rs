use std::time::Instant;

use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

use crate::server::{Server, session};
use crate::server::messages::BroadcastMessage;
use crate::pusher::Pusher;
use std::sync::{Arc, Mutex};
use crate::server::errors::{WsrsErrorKind, WsrsError};

/// Entry point for our websocket route
pub async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let app: String = req.match_info().get("app").unwrap().parse().unwrap();

    ws::start(
        session::Session {
            id: 0,
            hb: Instant::now(),
            app,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}


pub async fn event(
    req: HttpRequest,
    info: web::Json<BroadcastMessage>,
    srv: web::Data<Addr<Server>>,
) -> impl Responder {
    let app: String = req.match_info().get("app").unwrap().parse().unwrap();

    let msg = BroadcastMessage {
        app,
        data: info.data.clone(),
        channels: info.channels.clone(),
        channel: info.channel.clone(),
        name: info.name.clone(),
        socket_id: info.socket_id,
    };

    srv.get_ref().clone().do_send(msg);

    format!("")
}

pub async fn get_channels(
    req: HttpRequest,
    srv: web::Data<Arc<Mutex<Pusher>>>,
) -> impl Responder {
    let app: String = req.match_info().get("app").unwrap().parse().unwrap();

    match srv.lock().unwrap().get_channels(&app) {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(e) => err_response(e),
    }
}

pub async fn get_channel_users(
    req: HttpRequest,
    srv: web::Data<Arc<Mutex<Pusher>>>,
) -> impl Responder {
    let app: String = req.match_info().get("app").unwrap().parse().unwrap();
    let channel: String = req.match_info().get("channel").unwrap().parse().unwrap();

    match srv.lock().unwrap().get_channel_users(app.as_str(), channel.as_str()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => err_response(e),
    }
}

fn err_response(e: WsrsError) -> HttpResponse {
    let payload = serde_json::json!({
        "error": e.message,
    });

    match e.kind {
        WsrsErrorKind::Other => {
            HttpResponse::BadRequest().json(payload)
        },
        WsrsErrorKind::ChannelNotFound | WsrsErrorKind::AppNotFound => {
            HttpResponse::NotFound().json(payload)
        }
    }
}
