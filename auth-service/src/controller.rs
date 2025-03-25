use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, post,
    web::{self, Data, Json, Path},
};
use common::{
    jwt::{
        external::{ExternalClaims, get_authenticated_user},
        internal::authenticate_internal_request,
    },
    utils::api_response::{ApiResponse, DocErrorApiResponse, DocSuccessApiResponse},
};
use mongodb::Database;
use utoipa::OpenApi;

use crate::{
    doc::ApiDoc,
    model::{Auth, CreateAuthRequest, LoginRequest, LoginResponse},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/auth")
        .service(health_check)
        .service(get_auths)
        .service(get_me)
        .service(register)
        .service(login)
        .service(delete_auth_by_user_id)
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
    path = "/api/auth/health",
    tag = "Public Endpoints",
    summary = "Check if Auth Service is alive",
    description = "Returns 200 if the Auth Service is up and running.",
    responses(
        (status = 200, description = "Auth Service is Alive", body = DocSuccessApiResponse<serde_json::Value>)
    ),
    security(
        ("public_routes" = [])
    )
)]
#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Auth Service is Alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/auth",
    tag = "Internal Endpoints",
    summary = "Retrieve all credentials",
    description = "Used internally to list all credentials. Restricted to internal requests using an internal JWT.",
    responses(
        (status = 200, description = "All credentials were successfully retrieved.", body = DocSuccessApiResponse<Vec<Auth>>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the credentials retrieval.", body = DocErrorApiResponse)
    )
)]
#[get("")]
async fn get_auths(db: Data<Database>, req: HttpRequest) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(jwt_payload) => jwt_payload,
        Err(err_res) => return err_res,
    };

    match service::get_auths(&db).await {
        Ok(auths) => {
            let response: ApiResponse<Vec<Auth>> =
                ApiResponse::success("All credentials were successfully retrieved.", Some(auths));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the credentials retrieving.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "Protected Endpoints",
    summary = "Get current user credentials",
    description = "Returns the roles and user ID for the currently authenticated user.",
    responses(
        (status = 200, description = "User successfully retrieved", body = DocSuccessApiResponse<ExternalClaims>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the retrieval.", body = DocErrorApiResponse)
    )
)]
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

#[utoipa::path(
    delete,
    path = "/api/auth/{user_id}",
    tag = "Internal Endpoints",
    summary = "Delete user credentials",
    description = "Deletes the authentication credentials associated with the specified user. This endpoint is called when a user is deleted to ensure that their authentication data is also removed. Access is restricted to internal requests using an internal JWT.",
    responses(
        (status = 200, description = "User credentials were successfully deleted.", body = DocSuccessApiResponse<Auth>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the deletion of user credentials.", body = DocErrorApiResponse)
    ),
    params(
        ("user_id" = String, Path, description = "The ID of the user whose credentials should be deleted")
    )
)]
#[delete("/{user_id}")]
async fn delete_auth_by_user_id(
    db: Data<Database>,
    user_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(jwt_payload) => jwt_payload,
        Err(err_res) => return err_res,
    };

    let user_id = user_id.into_inner();

    match service::delete_auth_by_user_id(&db, user_id).await {
        Ok(auths) => {
            let response: ApiResponse<Auth> =
                ApiResponse::success("User credential were successfully deleted.", Some(auths));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the delete of user credentials.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Public Endpoints",
    summary = "Register a new user",
    description = "Creates new credentials (email/password, roles, etc.). May also notify the Users Service to create a user profile.",
    request_body = CreateAuthRequest,
    responses(
        (status = 200, description = "User successfully registered.", body = DocSuccessApiResponse<Auth>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the registration.", body = DocErrorApiResponse)
    ),
    security(
        ("public_routes" = [])
    )
)]
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

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Public Endpoints",
    summary = "User login",
    description = "Authenticates with email/password. Returns a JWT token upon success.",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Welcome back!", body = DocSuccessApiResponse<LoginResponse>),
        (status = 401, description = "Invalid credentials or an error occurred.", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during login.", body = DocErrorApiResponse)
    ),
    security(
        ("public_routes" = [])
    )
)]
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
