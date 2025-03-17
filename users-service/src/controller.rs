use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, post, put,
    web::{self, Data, Json, Path},
};
use common::{
    jwt::{
        external::{ExternalClaims, get_authenticated_user, user_has_any_of_these_roles},
        internal::authenticate_internal_request,
    },
    models::AuthRole,
    utils::api_response::{ApiResponse, ObjectIdToString},
};
use mongodb::Database;

use crate::{
    model::{CreateUserRequest, GetUserIdByEmailRequest, UpdateUserRequest, User},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(health_check)
        .service(get_users)
        .service(get_me)
        .service(get_user_id_by_email)
        .service(get_user_by_id)
        .service(create_user)
        .service(update_me)
        .service(update_user_by_id)
        .service(delete_user);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Server is Alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[get("")]
async fn get_users(db: Data<Database>, req: HttpRequest) -> impl Responder {
    let required_roles = &[AuthRole::Admin, AuthRole::Operator];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    match service::get_users(&db).await {
        Ok(users) => {
            let response: ApiResponse<Vec<User>> =
                ApiResponse::success("Users have been successfully recovered", Some(users));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("An error occured while retrieving users.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[get("/id-by-email")]
async fn get_user_id_by_email(
    db: Data<Database>,
    payload: Json<GetUserIdByEmailRequest>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let data = payload.into_inner();
    match service::get_user_id_by_email(&db, data).await {
        Ok(id) => {
            let response: ApiResponse<ObjectIdToString> = ApiResponse::success(
                "User successfully retrieved.",
                Some(ObjectIdToString { id: id.to_string() }),
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to retrieve the user with his email.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[get("/me")]
async fn get_me(db: Data<Database>, req: HttpRequest) -> impl Responder {
    let jwt_payload = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };
    let id = jwt_payload.user_id;

    match service::get_user_by_id(&db, id).await {
        Ok(user) => {
            let response: ApiResponse<User> =
                ApiResponse::success("User successfully retrieved", Some(user));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to retrieve the user", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[get("/{id}")]
async fn get_user_by_id(db: Data<Database>, id: Path<String>, req: HttpRequest) -> impl Responder {
    let ExternalClaims { role, user_id, .. } = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };
    let id = id.into_inner();

    if !(role == AuthRole::Admin || role == AuthRole::Operator) && user_id != id {
        let response: ApiResponse<()> = ApiResponse::error(
            "Access denied: insufficient role",
            "User is not allowed to access another profile",
        );
        return HttpResponse::Unauthorized().json(response);
    }

    match service::get_user_by_id(&db, id).await {
        Ok(user) => {
            let response: ApiResponse<User> =
                ApiResponse::success("User successfully retrieved", Some(user));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to retrieve the user", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[post("")]
async fn create_user(
    db: Data<Database>,
    payload: Json<CreateUserRequest>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let data = payload.into_inner();
    match service::create_user(&db, data).await {
        Ok(user) => {
            let response: ApiResponse<User> =
                ApiResponse::success("User created successfully", Some(user));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to create user", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[put("/me")]
async fn update_me(
    db: Data<Database>,
    payload: Json<UpdateUserRequest>,
    req: HttpRequest,
) -> impl Responder {
    let jwt_payload = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };

    let id = jwt_payload.user_id;
    let data = payload.into_inner();

    match service::update_user_by_id(&db, id, data).await {
        Ok(user) => {
            let response: ApiResponse<User> =
                ApiResponse::success("User successfully updated.", Some(user));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error("An error occurred.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[put("/{id}")]
async fn update_user_by_id(
    db: Data<Database>,
    id: Path<String>,
    payload: Json<UpdateUserRequest>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let id = id.into_inner();
    let data = payload.into_inner();

    match service::update_user_by_id(&db, id, data).await {
        Ok(user) => {
            let response: ApiResponse<User> =
                ApiResponse::success("User successfully updated.", Some(user));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error("An error occurred.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[delete("/{id}")]
async fn delete_user(db: Data<Database>, id: Path<String>, req: HttpRequest) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let id = id.into_inner();

    match service::delete_user(&db, id).await {
        Ok(user) => {
            let response: ApiResponse<User> =
                ApiResponse::success("User was successfully deleted.", Some(user));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error("An error occurred", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
