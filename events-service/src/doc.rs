use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{Modify, OpenApi, openapi::security::SecurityScheme};

use crate::controller::{
    __path_create_event, __path_delete_event_by_id, __path_get_event_by_id, __path_get_events,
    __path_health_check, __path_update_event_by_id, __path_update_event_seats_by_id,
};
use crate::model::{CreateEventRequest, Event, UpdateEventRequest, UpdateSeatsRequest};
use common::models::AuthRole;

// =============================================================================================================================

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Events Service",
        version = "1.0.0",
        description = r#"
The Events Service manages event data including title, description, date, capacity, and location.
It provides endpoints for retrieving, creating, updating, and deleting events, as well as updating the remaining seats.
"#
    ),
    paths(
        health_check,
        get_events,
        get_event_by_id,
        create_event,
        update_event_by_id,
        update_event_seats_by_id,
        delete_event_by_id
    ),
    components(
        schemas(
            Event,
            CreateEventRequest,
            UpdateEventRequest,
            UpdateSeatsRequest,
            AuthRole
        )
    ),
    security(
        ( ),
        ("public_routes" = ["read:items", "edit:items"]),
        ("bearerAuth" = [])
    ),
    modifiers(&SecurityAddon),
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
                    .build(),
            ),
        )
    }
}

// =============================================================================================================================
