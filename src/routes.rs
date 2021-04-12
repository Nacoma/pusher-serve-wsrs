use std::time::Instant;

use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

use crate::models::ClientEvent;
use crate::server;
use crate::session::WsChatSession;

/// Entry point for our websocket route
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::PusherServer>>,
) -> Result<HttpResponse, Error> {
    let app: String = req.match_info().get("app").unwrap().parse().unwrap();

    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            app,
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

pub async fn event(
    req: HttpRequest,
    info: web::Json<ClientEvent>,
    srv: web::Data<Addr<server::PusherServer>>,
) -> impl Responder {
    let app: String = req.match_info().get("app").unwrap().parse().unwrap();

    let msg = ClientEvent {
        app,
        data: info.data.clone(),
        channels: info.channels.clone(),
        channel: info.channel.clone(),
        name: info.name.clone(),
        socket_id: info.socket_id,
    };

    println!("{:?}", msg);

    srv.get_ref().clone().do_send(msg);

    format!("")
}
