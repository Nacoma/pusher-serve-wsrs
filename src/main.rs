#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;

use clap::{AppSettings, Clap};
use serde::Deserialize;

use crate::pusher::Pusher;
use crate::repository::Repository;
use crate::server::{routes, Server};
use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::{Arc, Mutex};

mod models;
mod pusher;
mod repository;
mod schema;
mod server;

#[derive(Clap)]
#[clap(version = "0.1", author = "Cody Mann <nathancodymann@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    //
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

fn establish_connection() -> SqliteConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&db_url).expect(&format!("Error connected to {}", db_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let port: i32 = match env::var("PORT") {
        Ok(v) => v.parse::<i32>().unwrap(),
        Err(_) => 6001,
    };

    let pusher = Arc::new(Mutex::new(Pusher::new(Repository::new(
        establish_connection(),
    ))));

    let server = Server::new(pusher.clone()).start();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .data(server.clone())
            .data(pusher.clone())
            .service(web::resource("/app/{app}").to(routes::ws::connect))
            .route("/apps/{app}/events", web::post().to(routes::ws::event))
            .route(
                "/apps/{app}/channels",
                web::get().to(routes::ws::get_channels),
            )
            .route(
                "/apps/{app}/channels/{channel}/users",
                web::get().to(routes::ws::get_channel_users),
            )
            .route("/api/apps", web::post().to(routes::apps::store))
            .route("/api/apps", web::get().to(routes::apps::index))
            .route("/api/apps/{app}", web::delete().to(routes::apps::delete))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
