use crate::controller::{
    __path_active_ticket_by_id, __path_cancel_ticket_by_id, __path_create_ticket,
    __path_delete_ticket_by_id, __path_get_ticket_by_id, __path_get_tickets, __path_health_check,
    __path_refund_ticket_by_id, __path_update_ticket_seat_number_by_id,
};
use crate::model::{CreateTicketRequest, Ticket, UpdateTicketSeatNumberByIdRequest};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{
    openapi::security::SecurityScheme,
    Modify, OpenApi,
};

// =============================================================================================================================

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Tickets Service",
        version = "1.0.0",
        description = r#"
The Tickets Service handles ticket purchases and management.
It provides endpoints for retrieving, creating, updating, activating, cancelling, refunding, and deleting tickets.
"#
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
    security(
        (),
        ("public_routes" = ["read:items", "edit:items"]),
        ("bearerAuth" = []),
    ),
    modifiers(&SecurityAddon),
    components(
        schemas(CreateTicketRequest, Ticket, UpdateTicketSeatNumberByIdRequest),
        
    )
)]
pub struct ApiDoc;

// =============================================================================================================================

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            )
        )
    }
}

// =============================================================================================================================