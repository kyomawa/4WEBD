use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{Modify, OpenApi, openapi::security::SecurityScheme};

use crate::controller::{
    __path_create_user, __path_delete_user, __path_get_me, __path_get_user_by_id,
    __path_get_user_id_by_email, __path_get_users, __path_health_check, __path_update_me,
    __path_update_user_by_id,
};
use crate::model::{CreateUserRequest, GetUserIdByEmailRequest, UpdateUserRequest, User};
use common::models::AuthRole;

// =============================================================================================================================

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Users Service",
        version = "1.0.0",
        description = r#"
The Users Service manages user profile data such as name, email, and contact information.
It provides endpoints for retrieving, creating, updating, and deleting user profiles.
"#
    ),
    paths(
        health_check,
        get_users,
        get_user_id_by_email,
        get_me,
        get_user_by_id,
        create_user,
        update_me,
        update_user_by_id,
        delete_user
    ),
    components(
        schemas(
            User,
            GetUserIdByEmailRequest,
            CreateUserRequest,
            UpdateUserRequest,
            AuthRole,
        )
    ),
    security(
      (),
      ("public_routes" = ["read:items", "edit:items"]),
      ("bearerAuth" = []),
    ),
    modifiers(&SecurityAddon)
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
