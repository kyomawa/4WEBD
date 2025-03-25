use common::jwt::external::ExternalClaims;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{Modify, OpenApi, openapi::security::SecurityScheme};

use crate::controller::{
    __path_get_auths, __path_get_me, __path_health_check, __path_login, __path_register,
};
use crate::model::{Auth, CreateAuthRequest, LoginRequest, LoginResponse};
use common::models::AuthRole;

// =============================================================================================================================

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth Service",
        version = "1.0.0",
        description = r#"
The Auth Service handles user authentication (login, token management) and basic authorization checks.
It provides public endpoints for registering and logging in,
and internal endpoints for the management of credentials.
"#
    ),
    paths(
        health_check,
        get_auths,
        get_me,
        register,
        login
    ),
    components(
        schemas(
            Auth,
            CreateAuthRequest,
            LoginRequest,
            LoginResponse,
            AuthRole,
            ExternalClaims
        )
    ),
    security(
        (),
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
