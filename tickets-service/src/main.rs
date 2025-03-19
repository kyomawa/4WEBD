use actix_web::{App, HttpServer, web};
use controller::config;
use db::init_db;
use extractor::deserialize_error_extractor;

mod controller;
mod db;
mod extractor;
mod model;
mod service;

// =============================================================================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db = init_db().await.expect("❌ Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(deserialize_error_extractor())
            .configure(config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// =============================================================================================================================
