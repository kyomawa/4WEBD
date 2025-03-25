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
    utils::api_response::{
        ApiResponse, DocErrorApiResponse, DocSuccessApiResponse, ObjectIdToString,
    },
};
use mongodb::Database;
use utoipa::OpenApi;

use crate::{
    doc::ApiDoc,
    model::{CreateUserRequest, GetUserIdByEmailRequest, UpdateUserRequest, User},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/users")
        .service(health_check)
        .service(get_users)
        .service(get_me)
        .service(get_user_id_by_email)
        .service(get_user_by_id)
        .service(create_user)
        .service(update_me)
        .service(update_user_by_id)
        .service(delete_user)
        .service(web::resource("/doc").route(web::get().to(|| async {
            HttpResponse::Found()
                .append_header(("Location", "./"))
                .finish()
        })))
        .service(web::scope("/doc").service(
            utoipa_swagger_ui::SwaggerUi::new("{_:.*}").url("openapi.json", ApiDoc::openapi()),
        ));

    cfg.service(scope);
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/users/health",
    tag = "Public Endpoints",
    summary = "Check if Users Service is alive",
    description = "Returns 200 if the Users Service is running.",
    responses(
        (status = 200, description = "Users Service is Alive", body = DocSuccessApiResponse<serde_json::Value>)
    ),
    security(
        ("public_routes" = [])
    )
)]
#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Users Service is Alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "Protected Endpoints",
    summary = "Retrieve all users",
    description = "Fetches a list of all users. Access is restricted to Admin or Operator roles.",
    responses(
        (status = 200, description = "Users have been successfully retrieved", body = DocSuccessApiResponse<Vec<User>>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred while retrieving users", body = DocErrorApiResponse)
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/users/id-by-email",
    tag = "Internal Endpoints",
    summary = "Retrieve user ID by email",
    description = "Fetches the user ID corresponding to the provided email. This endpoint is for internal use.",
    request_body = GetUserIdByEmailRequest,
    responses(
        (status = 200, description = "User successfully retrieved", body = DocSuccessApiResponse<ObjectIdToString>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to retrieve the user by email", body = DocErrorApiResponse)
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Protected Endpoints",
    summary = "Retrieve current user profile",
    description = "Returns the profile of the currently authenticated user.",
    responses(
        (status = 200, description = "User successfully retrieved", body = DocSuccessApiResponse<User>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to retrieve the user", body = DocErrorApiResponse)
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "Protected Endpoints",
    summary = "Retrieve a user by ID",
    description = "Fetches the profile of a specific user. Access is restricted to Admin, Operator, or the user themselves.",
    responses(
        (status = 200, description = "User successfully retrieved", body = DocSuccessApiResponse<User>),
        (status = 401, description = "Access denied: insufficient role", body = DocErrorApiResponse),
        (status = 500, description = "Failed to retrieve the user", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "User ID")
    )
)]
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

#[utoipa::path(
    post,
    path = "/api/users",
    tag = "Protected Endpoints",
    summary = "Register a new user",
    description = "Creates a new user profile. This may be a two-step process if registration is handled separately from authentication.",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created successfully", body = DocSuccessApiResponse<User>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to create user", body = DocErrorApiResponse)
    )
)]
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

#[utoipa::path(
    put,
    path = "/api/users/me",
    tag = "Protected Endpoints",
    summary = "Update current user's profile",
    description = "Updates the profile of the currently authenticated user.",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User successfully updated", body = DocSuccessApiResponse<User>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred", body = DocErrorApiResponse)
    )
)]
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

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    tag = "Protected Endpoints",
    summary = "Update a user's profile",
    description = "Updates the profile of a specific user. Access is restricted to Admin users.",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User successfully updated", body = DocSuccessApiResponse<User>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "User ID")
    )
)]
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

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "Protected Endpoints",
    summary = "Delete a user",
    description = "Permanently deletes a user specified by its ID. Access is restricted to Admin users.",
    responses(
        (status = 200, description = "User was successfully deleted", body = DocSuccessApiResponse<User>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "User ID")
    )
)]
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
