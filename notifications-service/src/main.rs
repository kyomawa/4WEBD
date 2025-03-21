use actix_web::{App, HttpServer, web};
use controller::config;
use cron_jobs::cron_jobs;
use db::init_db;
use extractor::deserialize_error_extractor;

mod controller;
mod cron_jobs;
mod db;
mod email;
mod extractor;
mod model;
mod service;

// =============================================================================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = init_db().await.expect("❌ Failed to connect to database");
    let db_for_cron = db.clone();

    actix_rt::spawn(async move {
        cron_jobs(db_for_cron).await;
    });

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
