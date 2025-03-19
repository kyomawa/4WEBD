use actix_web::{HttpResponse, Responder, get, web};
use common::utils::api_response::ApiResponse;

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/tickets").service(health_check);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Tickets Service is alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================
