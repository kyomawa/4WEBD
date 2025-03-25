use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, patch, post, put,
    web::{self, Data, Json, Path},
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
    model::{CreateEventRequest, Event, UpdateEventRequest, UpdateSeatsRequest},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/events")
        .service(health_check)
        .service(get_events)
        .service(get_event_by_id)
        .service(create_event)
        .service(update_event_by_id)
        .service(update_event_seats_by_id)
        .service(delete_event_by_id)
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
    path = "/api/events/health",
    responses(
        (status = 200, description = "Events Service is alive", body = DocSuccessApiResponse<serde_json::Value>)
    ),
    security(
        ("public_routes" = [])
    )
)]
#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Events Service is alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/events",
    responses(
        (status = 200, description = "Events were successfully retrieved.", body = DocSuccessApiResponse<Vec<Event>>),
        (status = 500, description = "An error occurred while trying to get events.", body = DocErrorApiResponse)
    ),
    security(
        ("public_routes" = [])
    )
)]
#[get("")]
async fn get_events(db: Data<Database>) -> impl Responder {
    match service::get_events(&db).await {
        Ok(events) => {
            let response: ApiResponse<Vec<Event>> =
                ApiResponse::success("Events were successfully retrieved.", Some(events));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured while trying to get events.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/events/{id}",
    responses(
        (status = 200, description = "Event was successfully retrieved.", body = DocSuccessApiResponse<Event>),
        (status = 500, description = "Failed to retrieve the event by id", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Event id")
    ),
    security(
        ("public_routes" = [])
    )
)]
#[get("/{id}")]
async fn get_event_by_id(db: Data<Database>, id: Path<String>) -> impl Responder {
    let id = id.into_inner();
    match service::get_event_by_id(&db, id).await {
        Ok(event) => {
            let response: ApiResponse<Event> =
                ApiResponse::success("Event was successfully retrieved.", Some(event));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to retrieve the event by id", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    post,
    path = "/api/events",
    request_body = CreateEventRequest,
    responses(
        (status = 200, description = "Event was successfully created.", body = DocSuccessApiResponse<Event>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the event creation.", body = DocErrorApiResponse)
    )
)]
#[post("")]
async fn create_event(
    db: Data<Database>,
    payload: Json<CreateEventRequest>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin, AuthRole::EventCreator];
    let jwt_payload = match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let event = payload.into_inner();
    match service::create_event(&db, event, jwt_payload.user_id).await {
        Ok(event) => {
            let response: ApiResponse<Event> =
                ApiResponse::success("Event was successfully created.", Some(event));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("An error occured during the event creation.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    put,
    path = "/api/events/{id}",
    request_body = UpdateEventRequest,
    responses(
        (status = 200, description = "Event was successfully updated.", body = DocSuccessApiResponse<Event>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to update the event", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Event id")
    )
)]
#[put("/{id}")]
async fn update_event_by_id(
    db: Data<Database>,
    payload: Json<UpdateEventRequest>,
    id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin, AuthRole::EventCreator];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let id = id.into_inner();
    let payload = payload.into_inner();

    match service::update_event_by_id(&db, payload, id).await {
        Ok(event) => {
            let response: ApiResponse<Event> =
                ApiResponse::success("Event was successfully updated.", Some(event));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to update the event", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    patch,
    path = "/api/events/{id}/update-seats",
    request_body = UpdateSeatsRequest,
    responses(
        (status = 200, description = "Remaining seats successfully updated.", body = DocSuccessApiResponse<Event>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to update remaining seats.", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Event id")
    )
)]
#[patch("/{id}/update-seats")]
async fn update_event_seats_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
    payload: web::Json<UpdateSeatsRequest>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let event_id = id.into_inner();
    let delta = payload.delta;

    match service::update_event_seats_by_id(&db, event_id, delta).await {
        Ok(event) => {
            let response: ApiResponse<Event> =
                ApiResponse::success("Remaining seats successfully updated.", Some(event));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to update remaining seats.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    delete,
    path = "/api/events/{id}",
    responses(
        (status = 200, description = "Event was successfully deleted.", body = DocSuccessApiResponse<Event>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to delete the event.", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Event id")
    )
)]
#[delete("/{id}")]
async fn delete_event_by_id(
    db: Data<Database>,
    id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin, AuthRole::EventCreator];
    let jwt_payload = match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let id = id.into_inner();
    match service::delete_event_by_id(&db, jwt_payload.user_id, jwt_payload.role, id).await {
        Ok(event) => {
            let response: ApiResponse<Event> =
                ApiResponse::success("Event was successfully deleted.", Some(event));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to delete the event.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
