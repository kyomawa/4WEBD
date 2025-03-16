use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Data, Json},
};
use common::utils::api_response::ApiResponse;
use mongodb::Database;

use crate::{
    model::{Auth, CreateAuthRequest},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/auth").service(health_check).service(register);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Server is Alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[post("/register")]
async fn register(db: Data<Database>, payload: Json<CreateAuthRequest>) -> impl Responder {
    let data = payload.into_inner();

    match service::register(&db, data).await {
        Ok(credentials) => {
            let response: ApiResponse<Auth> =
                ApiResponse::success("User successfully registered.", Some(credentials));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("An error occured during the registering.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
