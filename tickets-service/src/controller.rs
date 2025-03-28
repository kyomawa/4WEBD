use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, patch, post,
    web::{self, Data, Json, Path},
};
use common::{
    jwt::{
        external::{get_authenticated_user, user_has_any_of_these_roles},
        internal::authenticate_internal_request,
    },
    models::AuthRole,
    utils::api_response::{ApiResponse, DocErrorApiResponse, DocSuccessApiResponse},
};
use mongodb::Database;
use utoipa::OpenApi;

use crate::{
    doc::ApiDoc,
    model::{CreateTicketRequest, Ticket, UpdateTicketSeatNumberByIdRequest},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/tickets")
        .service(health_check)
        .service(get_tickets)
        .service(web::resource("/doc").route(web::get().to(|| async {
            HttpResponse::Found()
                .append_header(("Location", "./"))
                .finish()
        })))
        .service(web::scope("/doc").service(
            utoipa_swagger_ui::SwaggerUi::new("{_:.*}").url("openapi.json", ApiDoc::openapi()),
        ))
        .service(get_ticket_by_id)
        .service(create_ticket)
        .service(update_ticket_seat_number_by_id)
        .service(active_ticket_by_id)
        .service(cancel_ticket_by_id)
        .service(refund_ticket_by_id)
        .service(delete_ticket_by_id);

    cfg.service(scope);
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/tickets/health",
    tag = "Public Endpoints",
    summary = "Check if Tickets Service is alive",
    description = "Returns 200 if the Tickets Service is up and running.",
    responses(
        (status = 200, description = "Tickets Service is alive", body = DocSuccessApiResponse<serde_json::Value>)
    ),
    security(
        ("public_routes" = [])
    )
)]
#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Tickets Service is alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/tickets",
    tag = "Protected Endpoints",
    summary = "Retrieve all tickets",
    description = "Fetches a list of tickets for the currently authenticated user. Admins and Operators see all tickets.",
    responses(
        (status = 200, description = "Tickets were successfully retrieved.", body = DocSuccessApiResponse<Vec<Ticket>>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to retrieve tickets.", body = DocErrorApiResponse)
    )
)]
#[get("")]
async fn get_tickets(db: Data<Database>, req: HttpRequest) -> impl Responder {
    let jwt_payload = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };

    match service::get_tickets(&db, jwt_payload.user_id, jwt_payload.role).await {
        Ok(tickets) => {
            let response: ApiResponse<Vec<Ticket>> =
                ApiResponse::success("Tickets were successfully retrieved.", Some(tickets));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to retrieve tickets.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/tickets/{ticket_id}",
    tag = "Protected Endpoints",
    summary = "Retrieve a ticket by ID",
    description = "Fetches details of a specific ticket by its ID. Access is restricted based on user role or ownership.",
    responses(
        (status = 200, description = "Ticket was successfully retrieved.", body = DocSuccessApiResponse<Ticket>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to retrieve the ticket.", body = DocErrorApiResponse)
    ),
    params(
        ("ticket_id" = String, Path, description = "Ticket ID")
    )
)]
#[get("/{ticket_id}")]
async fn get_ticket_by_id(
    db: Data<Database>,
    ticket_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let jwt_payload = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };

    let ticket_id = ticket_id.into_inner();

    match service::get_ticket_by_id(&db, ticket_id, jwt_payload.role, jwt_payload.user_id).await {
        Ok(ticket) => {
            let response: ApiResponse<Ticket> =
                ApiResponse::success("The ticket was successfully retrieved.", Some(ticket));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to retrieve the ticket.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    post,
    path = "/api/tickets",
    tag = "Protected Endpoints",
    summary = "Create a new ticket",
    description = "Creates a new ticket for an event. Requires the user to be authenticated.",
    request_body = CreateTicketRequest,
    responses(
        (status = 200, description = "The ticket was successfully created.", body = DocSuccessApiResponse<Ticket>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to create the ticket.", body = DocErrorApiResponse)
    )
)]
#[post("")]
async fn create_ticket(
    db: Data<Database>,
    ticket_data: Json<CreateTicketRequest>,
    req: HttpRequest,
) -> impl Responder {
    match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };

    let ticket_data = ticket_data.into_inner();

    match service::create_ticket(&db, ticket_data).await {
        Ok(ticket) => {
            let response: ApiResponse<Ticket> =
                ApiResponse::success("The ticket was successfully created.", Some(ticket));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to create the ticket.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    patch,
    path = "/api/tickets/{ticket_id}/seat",
    tag = "Protected Endpoints",
    summary = "Update ticket seat number",
    description = "Updates the seat number of a ticket. Requires the user to be authenticated.",
    request_body = UpdateTicketSeatNumberByIdRequest,
    responses(
        (status = 200, description = "The ticket seat number was successfully updated.", body = DocSuccessApiResponse<Ticket>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to update the ticket seat number.", body = DocErrorApiResponse)
    ),
    params(
        ("ticket_id" = String, Path, description = "Ticket ID")
    )
)]
#[patch("/{ticket_id}/seat")]
async fn update_ticket_seat_number_by_id(
    db: Data<Database>,
    ticket_data: Json<UpdateTicketSeatNumberByIdRequest>,
    ticket_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let jwt_payload = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };

    let ticket_id = ticket_id.into_inner();
    let ticket_data = ticket_data.into_inner();

    match service::update_ticket_seat_number_by_id(
        &db,
        ticket_data,
        jwt_payload.user_id,
        jwt_payload.role,
        ticket_id,
    )
    .await
    {
        Ok(ticket) => {
            let response: ApiResponse<Ticket> = ApiResponse::success(
                "The ticket seat number was successfully updated.",
                Some(ticket),
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to update the ticket seat number.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    patch,
    path = "/api/tickets/{ticket_id}/active",
    tag = "Internal Endpoints",
    summary = "Activate a ticket",
    description = "Activates a ticket by changing its status to Active. This is an internal operation.",
    responses(
        (status = 200, description = "The ticket was successfully activated.", body = DocSuccessApiResponse<Ticket>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to activate the ticket.", body = DocErrorApiResponse)
    )
)]
#[patch("/{ticket_id}/active")]
async fn active_ticket_by_id(
    db: Data<Database>,
    ticket_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let ticket_id = ticket_id.into_inner();

    match service::active_ticket_by_id(&db, ticket_id).await {
        Ok(ticket) => {
            let response: ApiResponse<Ticket> =
                ApiResponse::success("The ticket was successfully activated.", Some(ticket));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to activate the ticket.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    patch,
    path = "/api/tickets/{ticket_id}/cancel",
    tag = "Protected Endpoints",
    summary = "Cancel a ticket",
    description = "Cancels a ticket by updating its status to Cancelled. Accessible by the ticket owner or an administrator.",
    responses(
        (status = 200, description = "The ticket was successfully cancelled.", body = DocSuccessApiResponse<Ticket>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to cancel the ticket.", body = DocErrorApiResponse)
    ),
    params(
        ("ticket_id" = String, Path, description = "Ticket ID")
    )
)]
#[patch("/{ticket_id}/cancel")]
async fn cancel_ticket_by_id(
    db: Data<Database>,
    ticket_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let jwt_payload = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };

    let ticket_id = ticket_id.into_inner();

    match service::cancel_ticket_by_id(&db, ticket_id, jwt_payload.role, jwt_payload.user_id).await
    {
        Ok(ticket) => {
            let response: ApiResponse<Ticket> =
                ApiResponse::success("The ticket was successfully cancelled.", Some(ticket));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to cancel the ticket.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    patch,
    path = "/api/tickets/{ticket_id}/refund",
    tag = "Protected Endpoints",
    summary = "Refund a ticket",
    description = "Refunds a ticket by updating its status to Refunded. Accessible by the ticket owner or an administrator.",
    responses(
        (status = 200, description = "The ticket was successfully refunded.", body = DocSuccessApiResponse<Ticket>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to refund the ticket.", body = DocErrorApiResponse)
    ),
    params(
        ("ticket_id" = String, Path, description = "Ticket ID")
    )
)]
#[patch("/{ticket_id}/refund")]
async fn refund_ticket_by_id(
    db: Data<Database>,
    ticket_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let jwt_payload = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };

    let ticket_id = ticket_id.into_inner();

    match service::refund_ticket_by_id(&db, ticket_id, jwt_payload.role, jwt_payload.user_id).await
    {
        Ok(ticket) => {
            let response: ApiResponse<Ticket> =
                ApiResponse::success("The ticket was successfully refunded.", Some(ticket));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to refund the ticket.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    delete,
    path = "/api/tickets/{ticket_id}",
    tag = "Protected Endpoints",
    summary = "Delete a ticket",
    description = "Permanently deletes a ticket specified by its ID. This action is restricted to administrators.",
    responses(
        (status = 200, description = "The ticket was successfully deleted.", body = DocSuccessApiResponse<Ticket>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "Failed to delete the ticket.", body = DocErrorApiResponse)
    ),
    params(
        ("ticket_id" = String, Path, description = "Ticket ID")
    )
)]
#[delete("/{ticket_id}")]
async fn delete_ticket_by_id(
    db: Data<Database>,
    ticket_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(claims) => claims,
        Err(err_res) => return err_res,
    };

    let ticket_id = ticket_id.into_inner();

    match service::delete_ticket_by_id(&db, ticket_id).await {
        Ok(ticket) => {
            let response: ApiResponse<Ticket> =
                ApiResponse::success("The ticket was successfully deleted.", Some(ticket));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("Failed to delete the ticket.", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
