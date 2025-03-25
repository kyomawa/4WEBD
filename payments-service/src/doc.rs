use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{Modify, OpenApi, openapi::security::SecurityScheme};

use crate::controller::{
    __path_create_payment, __path_delete_payment_by_id, __path_get_payment_by_id,
    __path_get_payments, __path_health_check, __path_update_payment_status_by_id,
};
use crate::model::{CreatePaymentRequest, Payment, PaymentStatus, UpdatePaymentStatusByIdRequest};
use common::models::AuthRole;

// =============================================================================================================================

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        get_payments,
        get_payment_by_id,
        create_payment,
        update_payment_status_by_id,
        delete_payment_by_id
    ),
    components(
        schemas(
            Payment,
            CreatePaymentRequest,
            UpdatePaymentStatusByIdRequest,
            PaymentStatus,
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
        (name = "Payments", description = "Endpoints for handling payments")
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
