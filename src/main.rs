use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use serde::Deserialize;
use clap::{AppSettings, Clap};

use std::env;
use actix::{Actor, Addr};
use actix_web::{App, HttpServer, web};

mod session;
mod server;
mod models;
mod routes;

#[derive(Clap)]
#[clap(version = "0.1", author = "Cody Mann <nathancodymann@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    config_file_path: String,
}


#[derive(Deserialize)]
struct Conf {
    port: Option<u16>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let conf_file = std::fs::read_to_string(opts.config_file_path).unwrap();

    let conf: Conf = toml::from_str(conf_file.as_str()).unwrap();

    let port = match conf.port {
        Some(p) => p,
        None => 6001,
    };

    let app_state = Arc::new(AtomicUsize::new(0));

    let server = server::PusherServer::new(app_state.clone()).start();

    HttpServer::new(move ||
        App::new()
            .data(app_state.clone())
            .data(server.clone())
            .service(web::resource("/app/{app}").to(routes::chat_route))
            .route("/apps/{app}/events", web::post().to(routes::event))
    )
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}
