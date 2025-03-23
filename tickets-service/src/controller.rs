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
    utils::api_response::ApiResponse,
};
use mongodb::Database;

use crate::{
    model::{CreateTicketRequest, Ticket, UpdateTicketSeatNumberByIdRequest},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/tickets")
        .service(health_check)
        .service(get_tickets)
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

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Tickets Service is alive", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

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
