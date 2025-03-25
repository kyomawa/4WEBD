use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{Modify, OpenApi, openapi::security::SecurityScheme};

use crate::controller::{
    __path_create_backup, __path_delete_backup_by_id, __path_get_backup_by_id,
    __path_get_last_backup_by_service_name, __path_health_check,
};
use crate::model::{Backup, CreateBackup, GetLastBackupByServiceName};
use common::models::AuthRole;

// =============================================================================================================================

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Backups Service",
        version = "1.0.0",
        description = r#"
The Backups Service centralizes the management and scheduling of backups for various microservices.
It provides endpoints for retrieving, creating, and deleting backups.
"#
    ),
    paths(
        health_check,
        get_last_backup_by_service_name,
        get_backup_by_id,
        create_backup,
        delete_backup_by_id
    ),
    components(
        schemas(
            Backup,
            CreateBackup,
            GetLastBackupByServiceName,
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
