use actix_web::{
    HttpRequest, HttpResponse, Responder, get,
    web::{self, Data, Path, ServiceConfig},
};
use common::{
    jwt::{external::user_has_any_of_these_roles, internal::authenticate_internal_request},
    models::AuthRole,
    utils::api_response::ApiResponse,
};
use mongodb::Database;

use crate::{
    model::{Backup, GetLastBackupByServiceName},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut ServiceConfig) {
    let scope = web::scope("/api/backups")
        .service(health_check)
        .service(get_last_backup_by_service_name)
        .service(get_backup_by_id);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Backups Service is Alive.", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[get("/{service_name}/last")]
async fn get_last_backup_by_service_name(
    db: Data<Database>,
    service_name: Path<GetLastBackupByServiceName>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(jwt_payload) => jwt_payload,
        Err(err_res) => return err_res,
    };

    let service_name = service_name.into_inner();

    match service::get_last_backup_by_service_name(&db, service_name).await {
        Ok(backup) => {
            let response: ApiResponse<Backup> =
                ApiResponse::success("Backup was successfully retrieved.", Some(backup));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured while retrieving the backup.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[get("/{id}")]
async fn get_backup_by_id(
    db: Data<Database>,
    backup_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(jwt_payload) => jwt_payload,
        Err(err_res) => return err_res,
    };

    let backup_id = backup_id.into_inner();

    match service::get_backup_by_id(&db, backup_id).await {
        Ok(backup) => {
            let response: ApiResponse<Backup> =
                ApiResponse::success("Backup was successfully retrieved.", Some(backup));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured while retrieving the backup.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
