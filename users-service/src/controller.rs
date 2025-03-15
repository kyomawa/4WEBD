use actix_web::{ get, post, web::{ self, Data, Json }, HttpResponse, Responder };
use mongodb::Database;

use crate::{ model::{ CreateUserRequest, User }, service, utils::ApiResponse };

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/users").service(health_check).service(create_user);

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

#[post("")]
async fn create_user(db: Data<Database>, payload: Json<CreateUserRequest>) -> impl Responder {
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
