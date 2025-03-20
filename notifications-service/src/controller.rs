use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};
use common::utils::api_response::ApiResponse;

// =============================================================================================================================

pub fn config(cfg: &mut ServiceConfig) {
    let scope = web::scope("/notifications").service(health_check);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Notifications Service is Alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================
