use actix_web::{ web, App, HttpServer };
use controller::config;

mod controller;
mod model;
mod service;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db = db::init_db().await.expect("❌ Failed to connect to database");

    HttpServer::new(move || App::new().app_data(web::Data::new(db.clone())).configure(config))
        .bind(("0.0.0.0", 8080))?
        .run().await
}
