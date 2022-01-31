use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use actix::{Actor, Addr};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::http::Error;
use actix_web::middleware::Logger;
use actix_web_actors::ws as actix_ws;
use crate::adapter::InMemoryAdapter;
use crate::app::App as PusherApp;
use crate::kind::WebSocket;
use crate::messages::OutgoingMessage;
use crate::repository::{AppRepo, InMemoryAppRepo};
use crate::ws::WebSocketHandler;
use crate::ws_handler::Session;

mod adapter;
mod app;
mod auth;
mod kind;
mod messages;
mod namespace;
mod socket;
mod ws;
mod ws_handler;
mod repository;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "trace");

    let adapter = Arc::new(Mutex::new(InMemoryAdapter::default()));

    let app = PusherApp::default();

    adapter.lock()
        .unwrap()
        .add_app(app.clone());

    let handler = WebSocketHandler::new(adapter.clone()).start();
    let repo: Arc<Mutex<dyn AppRepo>> = Arc::new(Mutex::new(InMemoryAppRepo::default()));

    repo.lock()
        .unwrap()
        .insert_app(app.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(handler.clone())
            .data(adapter.clone())
            .data(repo.clone())
            .service(web::resource("/app/{app}").to(connect))
            .service(api::events)
            .service(api::index)
    })
        .bind("0.0.0.0:9911")?
        .run()
        .await
}

async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    handler: web::Data<Addr<WebSocketHandler>>,
) -> impl Responder {
    let app_id: String = req.match_info().get("app").unwrap().parse().unwrap();

    let app = PusherApp::default();

    actix_ws::start(
        Session::new(app, handler.get_ref().clone()),
        &req,
        stream
    )
}

