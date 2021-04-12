use clap::{AppSettings, Clap};
use serde::Deserialize;

use actix::Actor;
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;

mod models;
mod routes;
mod server;
mod session;

#[derive(Clap)]
#[clap(version = "0.1", author = "Cody Mann <nathancodymann@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    config_file_path: String,
}

#[derive(Deserialize, Debug)]
struct Conf {
    port: Option<u16>,
    apps: Vec<ConfApp>,
}

#[derive(Deserialize, Debug)]
struct ConfApp {
    id: String,
    key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    let conf_file = std::fs::read_to_string(opts.config_file_path).unwrap();

    let conf: Conf = toml::from_str(conf_file.as_str()).unwrap();

    let port = match conf.port {
        Some(p) => p,
        None => 6001,
    };

    let mut app_keys: HashMap<String, String> = HashMap::new();

    for app in conf.apps {
        app_keys.insert(app.id, app.key);
    }

    let server = server::PusherServer::new(app_keys).start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(web::resource("/app/{app}").to(routes::chat_route))
            .route("/apps/{app}/events", web::post().to(routes::event))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
