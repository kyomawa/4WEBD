use actix_web::{App, HttpServer, web};
use controller::config;

mod controller;
mod db;
mod model;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db = db::init_db()
        .await
        .expect("❌ Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
