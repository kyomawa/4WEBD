use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post,
    web::{self, Data, Json, Path},
};
use common::{
    jwt::{
        external::{Role, user_has_any_of_these_roles},
        internal::authenticate_internal_request,
    },
    utils::api_response::ApiResponse,
};
use mongodb::Database;

use crate::{model::User, service};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(health_check)
        .service(get_users)
        .service(get_user_by_id)
        .service(create_user);

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
async fn get_users(db: Data<Database>, req: HttpRequest) -> impl Responder {
    let required_roles = &[Role::Admin, Role::Operator];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => {
            return err_res;
        }
    };

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

#[get("/{id}")]
async fn get_user_by_id(db: Data<Database>, id: Path<String>) -> impl Responder {
    let id = id.into_inner();
    match service::get_user_by_id(&db, id.as_str()).await {
        Ok(user) => {
            let response: ApiResponse<User> = ApiResponse::Success {
                success: true,
                message: "User successfully retrieved".to_string(),
                data: Some(user),
            };
            HttpResponse::InternalServerError().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::Error {
                success: false,
                message: "Failed to retrieve the user".to_string(),
                error: e.to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[post("")]
async fn create_user(db: Data<Database>, payload: Json<User>, req: HttpRequest) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(claims) => claims,
        Err(err_res) => {
            return err_res;
        }
    };

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
