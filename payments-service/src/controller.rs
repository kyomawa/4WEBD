use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};
use common::utils::api_response::ApiResponse;

// =============================================================================================================================

pub fn config(cfg: &mut ServiceConfig) {
    let scope = web::scope("/payments").service(health_check);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Payments Service is Alive.", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================
