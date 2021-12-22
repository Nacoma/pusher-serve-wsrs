use actix_web::{App, HttpServer};
use pusher::Pusher;
use actix::{Actor, ActorContext, Context};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
    })
        .bind("127.0.0.1:9001")?
        .run()
        .await
}
