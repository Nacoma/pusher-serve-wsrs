#[macro_use]
extern crate diesel;

#[macro_use]
extern crate pusher_message_derive;

use crate::adapter::InMemoryAdapter;
use crate::app::App as PusherApp;
use crate::kind::WebSocket;
use crate::messages::OutgoingMessage;
use crate::repository::sqlite::SqliteRepo;
use crate::repository::AppRepo;
use crate::ws::WebSocketHandler;
use crate::ws_handler::Session;
use actix::{Actor, Addr};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws as actix_ws;
use diesel::{Connection, SqliteConnection};
use serde::Deserialize;

use parking_lot::Mutex as PMutex;
use std::sync::Arc;

mod adapter;
mod api;
mod app;
mod auth;
mod kind;
mod messages;
mod namespace;
mod repository;
mod socket;
mod ws;
mod ws_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let repo: Arc<PMutex<dyn AppRepo>> = Arc::new(PMutex::new(SqliteRepo::new(
        SqliteConnection::establish("./tmp.db").unwrap(),
    )));

    let adapter = Arc::new(InMemoryAdapter::default());

    let handler = WebSocketHandler::new(adapter.clone(), repo.clone()).start();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(handler.clone())
            .data(adapter.clone())
            .data(repo.clone())
            .service(web::resource("/app/{app_id}").to(connect))
            .service(api::index)
            .service(api::apps::all)
            .service(api::apps::create)
            .service(api::events::publish)
    })
    .bind("0.0.0.0:9911")?
    .run()
    .await
}

#[derive(Deserialize)]
struct ConnectQuery {
    app_id: i64,
}

async fn connect(
    req: HttpRequest,
    query: web::Path<ConnectQuery>,
    stream: web::Payload,
    handler: web::Data<Addr<WebSocketHandler>>,
) -> impl Responder {
    actix_ws::start(
        Session::new(query.app_id, handler.get_ref().clone()),
        &req,
        stream,
    )
}
