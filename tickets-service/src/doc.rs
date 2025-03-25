use crate::controller::{
    __path_active_ticket_by_id, __path_cancel_ticket_by_id, __path_create_ticket,
    __path_delete_ticket_by_id, __path_get_ticket_by_id, __path_get_tickets, __path_health_check,
    __path_refund_ticket_by_id, __path_update_ticket_seat_number_by_id,
};
use crate::model::{CreateTicketRequest, Ticket, UpdateTicketSeatNumberByIdRequest};
use utoipa::OpenApi;

// =============================================================================================================================

#[derive(OpenApi)]
#[openapi(
    security(
        ("bearerAuth" = []),
    ),
    paths(
        health_check,
        get_tickets,
        get_ticket_by_id,
        create_ticket,
        update_ticket_seat_number_by_id,
        active_ticket_by_id,
        cancel_ticket_by_id,
        refund_ticket_by_id,
        delete_ticket_by_id
    ),
    components(
        schemas(CreateTicketRequest, Ticket, UpdateTicketSeatNumberByIdRequest),
        
    ),

    tags(
        (name = "Tickets", description = "Endpoints for managing tickets")
    )
)]
pub struct ApiDoc;

// =============================================================================================================================
