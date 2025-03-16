use actix_web::{
    HttpResponse, Responder, post,
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
    let scope = web::scope("/auth").service(register);

    cfg.service(scope);
}

// =============================================================================================================================

#[post("/register")]
async fn register(db: Data<Database>, payload: Json<CreateAuthRequest>) -> impl Responder {
    let data = payload.into_inner();

    match service::register(&db, data).await {
        Ok(credentials) => {
            let response: ApiResponse<Auth> = ApiResponse::Success {
                success: true,
                message: "User successfully registered.".to_string(),
                data: Some(credentials),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::Error {
                success: false,
                message: "An error occured during the registering.".to_string(),
                error: e.to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
