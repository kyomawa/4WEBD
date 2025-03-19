use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post,
    web::{self, Data, Json, Path},
};
use common::{jwt::external::get_authenticated_user, utils::api_response::ApiResponse};
use mongodb::Database;

use crate::{
    model::{CreateTicketRequest, Ticket},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/tickets")
        .service(health_check)
        .service(get_tickets)
        .service(get_ticket_by_id)
        .service(create_ticket);

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

#[get("/{id}")]
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
