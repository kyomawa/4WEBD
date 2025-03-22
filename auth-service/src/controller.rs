use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post,
    web::{self, Data, Json},
};
use common::{
    jwt::external::{ExternalClaims, get_authenticated_user},
    utils::api_response::ApiResponse,
};
use mongodb::Database;

use crate::{
    model::{Auth, CreateAuthRequest, LoginRequest, LoginResponse},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/auth")
        .service(health_check)
        .service(get_me)
        .service(register)
        .service(login);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Auth Service is Alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[get("/me")]
async fn get_me(req: HttpRequest) -> impl Responder {
    match get_authenticated_user(&req) {
        Ok(payload) => {
            let response: ApiResponse<ExternalClaims> =
                ApiResponse::success("User successfully retrieved", Some(payload));
            HttpResponse::Ok().json(response)
        }
        Err(err_res) => {
            return err_res;
        }
    }
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

#[post("/login")]
async fn login(db: Data<Database>, payload: Json<LoginRequest>) -> impl Responder {
    let data = payload.into_inner();

    match service::login(&db, data).await {
        Ok(token) => {
            let response: ApiResponse<LoginResponse> =
                ApiResponse::success("Welcome back !", Some(token));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Invalid credentials or an error occured.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
