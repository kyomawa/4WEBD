use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{Modify, OpenApi, openapi::security::SecurityScheme};

use crate::controller::{
    __path_create_notification, __path_delete_notification_by_id, __path_get_notification_by_id,
    __path_get_notifications, __path_health_check, __path_update_notification_status_by_id,
};
use crate::model::{
    CreateNotification, Notification, NotificationStatus, NotificationType,
    UpdateNotificationStatus,
};
use common::models::AuthRole;

// =============================================================================================================================

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        get_notifications,
        get_notification_by_id,
        create_notification,
        update_notification_status_by_id,
        delete_notification_by_id
    ),
    components(
        schemas(
            Notification,
            CreateNotification,
            UpdateNotificationStatus,
            NotificationType,
            NotificationStatus,
            AuthRole
        )
    ),
    security(
        ( ),
        ("public_routes" = ["read:items", "edit:items"]),
        ("bearerAuth" = [])
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Notifications", description = "Endpoints for managing notifications")
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
                    .build(),
            ),
        )
    }
}

// =============================================================================================================================
