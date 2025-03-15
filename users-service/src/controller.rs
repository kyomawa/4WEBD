use actix_web::{ get, post, web::{ self, Data, Json }, HttpResponse, Responder };
use common::utils::api_response::ApiResponse;
use mongodb::Database;

use crate::{ model::User, service };

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/users").service(health_check).service(get_users).service(create_user);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::Success {
        success: true,
        message: String::from("🟢 Server is Alive"),
        data: None,
    };
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[get("")]
async fn get_users(db: Data<Database>) -> impl Responder {
    match service::get_users(&db).await {
        Ok(users) => {
            let response: ApiResponse<Vec<User>> = ApiResponse::Success {
                success: true,
                message: "Users have been successfully recovered".to_string(),
                data: Some(users),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::Error {
                success: false,
                message: "An error occured while retrieving users.".to_string(),
                error: e.to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[post("")]
async fn create_user(db: Data<Database>, payload: Json<User>) -> impl Responder {
    let data = payload.into_inner();
    match service::create_user(&db, data).await {
        Ok(user) => {
            let response: ApiResponse<User> = ApiResponse::Success {
                success: true,
                message: String::from("User created successfully"),
                data: Some(user),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::Error {
                success: false,
                message: String::from("Failed to create user"),
                error: e.to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
