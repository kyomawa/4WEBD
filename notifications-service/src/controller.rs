use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, patch, post,
    web::{self, Data, Json, Path, ServiceConfig},
};
use common::{
    jwt::{external::user_has_any_of_these_roles, internal::authenticate_internal_request},
    models::AuthRole,
    utils::api_response::{ApiResponse, DocErrorApiResponse, DocSuccessApiResponse},
};
use mongodb::Database;
use utoipa::OpenApi;

use crate::{
    doc::ApiDoc,
    model::{CreateNotification, Notification, UpdateNotificationStatus},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut ServiceConfig) {
    let scope = web::scope("/api/notifications")
        .service(health_check)
        .service(get_notifications)
        .service(get_notification_by_id)
        .service(create_notification)
        .service(update_notification_status_by_id)
        .service(delete_notification_by_id)
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
    path = "/api/notifications/health",
    tag = "Public Endpoints",
    summary = "Check if Notifications Service is alive",
    description = "Returns 200 if the Notifications Service is up and running.",
    responses(
        (status = 200, description = "Notifications Service is Alive", body = DocSuccessApiResponse<serde_json::Value>)
    ),
    security(
        ("public_routes" = [])
    )
)]
#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Notifications Service is Alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/notifications",
    tag = "Protected Endpoints",
    summary = "Retrieve all notifications",
    description = "Fetches a list of all notifications. Access is restricted to Admin users.",
    responses(
        (status = 200, description = "All notifications were successfully retrieved.", body = DocSuccessApiResponse<Vec<Notification>>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the retrieval of notifications.", body = DocErrorApiResponse)
    )
)]
#[get("")]
async fn get_notifications(db: Data<Database>, req: HttpRequest) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    match service::get_notifications(&db).await {
        Ok(notifications) => {
            let response: ApiResponse<Vec<Notification>> = ApiResponse::success(
                "All notifications were successfully retrieved.",
                Some(notifications),
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the retrieving of notifications.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/notifications/{id}",
    tag = "Protected Endpoints",
    summary = "Retrieve a notification by ID",
    description = "Fetches the details of a notification specified by its ID. Access is restricted to Admin users.",
    responses(
        (status = 200, description = "The notification was successfully retrieved.", body = DocSuccessApiResponse<Notification>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the retrieval of the notification.", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Notification ID")
    )
)]
#[get("/{id}")]
async fn get_notification_by_id(
    db: Data<Database>,
    id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let id = id.into_inner();

    match service::get_notification_by_id(&db, id).await {
        Ok(notification) => {
            let response: ApiResponse<Notification> = ApiResponse::success(
                "The notification was successfully retrieved.",
                Some(notification),
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the retrieving of the notification.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    post,
    path = "/api/notifications",
    tag = "Internal Endpoints",
    summary = "Create a new notification",
    description = "Creates a new notification request. This endpoint is called internally after a successful ticket purchase.",
    request_body = CreateNotification,
    responses(
        (status = 200, description = "The notification was successfully created.", body = DocSuccessApiResponse<Notification>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the creation of the notification.", body = DocErrorApiResponse)
    )
)]
#[post("")]
async fn create_notification(
    db: Data<Database>,
    notification: Json<CreateNotification>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let notification = notification.into_inner();

    match service::create_notification(&db, notification).await {
        Ok(notification) => {
            let response: ApiResponse<Notification> = ApiResponse::success(
                "The notification was successfully created.",
                Some(notification),
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the creation of notification.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    patch,
    path = "/api/notifications/{id}",
    tag = "Protected Endpoints",
    summary = "Update notification status",
    description = "Updates the status of a notification (e.g., from pending to sent). Access is restricted to Admin users.",
    request_body = UpdateNotificationStatus,
    responses(
        (status = 200, description = "The notification was successfully updated.", body = DocSuccessApiResponse<Notification>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the update of the notification.", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Notification ID")
    )
)]
#[patch("/{id}")]
async fn update_notification_status_by_id(
    db: Data<Database>,
    notification: Json<UpdateNotificationStatus>,
    id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let id = id.into_inner();
    let notification = notification.into_inner();

    match service::update_notification_status_by_id(&db, id, notification).await {
        Ok(notification) => {
            let response: ApiResponse<Notification> = ApiResponse::success(
                "The notification was successfully updated.",
                Some(notification),
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the update of notification.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    delete,
    path = "/api/notifications/{id}",
    tag = "Protected Endpoints",
    summary = "Delete a notification",
    description = "Deletes a notification specified by its ID. Access is restricted to Admin users.",
    responses(
        (status = 200, description = "The notification was successfully deleted.", body = DocSuccessApiResponse<Notification>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the deletion of the notification.", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Notification ID")
    )
)]
#[delete("/{id}")]
async fn delete_notification_by_id(
    db: Data<Database>,
    id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let id = id.into_inner();

    match service::delete_notification_by_id(&db, id).await {
        Ok(notification) => {
            let response: ApiResponse<Notification> = ApiResponse::success(
                "The notification was successfully deleted.",
                Some(notification),
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the delete of notification.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
