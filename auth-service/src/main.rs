use actix_web::{ App, HttpServer };
use controller::config;

mod controller;
mod model;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config))
        .bind(("127.0.0.1", 8080))?
        .run().await
}
