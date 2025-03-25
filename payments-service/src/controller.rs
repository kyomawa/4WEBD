use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, patch, post,
    web::{self, Data, Json, Path, ServiceConfig},
};
use common::{
    jwt::{
        external::{ExternalClaims, get_authenticated_user, user_has_any_of_these_roles},
        internal::authenticate_internal_request,
    },
    models::AuthRole,
    utils::api_response::{ApiResponse, DocErrorApiResponse, DocSuccessApiResponse},
};
use mongodb::Database;
use utoipa::OpenApi;

use crate::{
    doc::ApiDoc,
    model::{CreatePaymentRequest, Payment, UpdatePaymentStatusByIdRequest},
    service,
};

// =============================================================================================================================

pub fn config(cfg: &mut ServiceConfig) {
    let scope = web::scope("/api/payments")
        .service(health_check)
        .service(get_payments)
        .service(get_payment_by_id)
        .service(create_payment)
        .service(update_payment_status_by_id)
        .service(delete_payment_by_id)
        .service(web::resource("/doc").route(web::get().to(|| async {
            HttpResponse::Found()
                .append_header(("Location", "./"))
                .finish()
        })))
        .service(web::scope("/doc").service(
            utoipa_swagger_ui::SwaggerUi::new("{_:.*}").url("openapi.json", ApiDoc::openapi()),
        ));

    cfg.service(scope);
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/payments/health",
    tag = "Public Endpoints",
    summary = "Check if Payments Service is alive",
    description = "Returns 200 if the Payments Service is running.",
    responses(
        (status = 200, description = "Payments Service is Alive.", body = DocSuccessApiResponse<serde_json::Value>)
    ),
    security(
        ("public_routes" = [])
    )
)]
#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Payments Service is Alive.", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/payments",
    tag = "Protected Endpoints",
    summary = "Retrieve all payments",
    description = "Fetches a list of all payments. Access is restricted to Admin users.",
    responses(
        (status = 200, description = "All payments were successfully retrieved.", body = DocSuccessApiResponse<Vec<Payment>>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the payments retrieval.", body = DocErrorApiResponse)
    ),
)]
#[get("")]
async fn get_payments(db: Data<Database>, req: HttpRequest) -> impl Responder {
    let required_roles = &[AuthRole::Admin];
    match user_has_any_of_these_roles(&req, required_roles) {
        Ok(jwt_payload) => jwt_payload,
        Err(err_res) => return err_res,
    };

    match service::get_payments(&db).await {
        Ok(payments) => {
            let response: ApiResponse<Vec<Payment>> =
                ApiResponse::success("All Payments were successfully retrieved.", Some(payments));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the payments retrieving.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    get,
    path = "/api/payments/{id}",
    tag = "Protected Endpoints",
    summary = "Retrieve a payment by ID",
    description = "Fetches the details of a specific payment by its ID. Access is restricted to authenticated users.",
    responses(
        (status = 200, description = "Payment was successfully retrieved.", body = DocSuccessApiResponse<Payment>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the payment retrieval.", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Payment ID")
    )
)]
#[get("/{id}")]
async fn get_payment_by_id(
    db: Data<Database>,
    payment_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let jwt_payload: ExternalClaims = match get_authenticated_user(&req) {
        Ok(payload) => payload,
        Err(err_res) => return err_res,
    };
    let payment_id = payment_id.into_inner();

    match service::get_payment_by_id(&db, payment_id, jwt_payload).await {
        Ok(payment) => {
            let response: ApiResponse<Payment> =
                ApiResponse::success("Payment was successfully retrieved.", Some(payment));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the payment retrieving.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    post,
    path = "/api/payments",
    tag = "Internal Endpoints",
    summary = "Create a new payment",
    description = "Creates a new payment for a ticket or event. This endpoint is intended for internal use.",
    request_body = CreatePaymentRequest,
    responses(
        (status = 200, description = "Payment was successfully created.", body = DocSuccessApiResponse<Payment>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the payment creation.", body = DocErrorApiResponse)
    )
)]
#[post("")]
async fn create_payment(
    db: Data<Database>,
    payment_data: Json<CreatePaymentRequest>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(jwt_payload) => jwt_payload,
        Err(err_res) => return err_res,
    };

    let payment_data = payment_data.into_inner();

    match service::create_payment(&db, payment_data).await {
        Ok(payment) => {
            let response: ApiResponse<Payment> =
                ApiResponse::success("Payment was successfully created.", Some(payment));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the payment creation.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    patch,
    path = "/api/payments/{id}",
    tag = "Internal Endpoints",
    summary = "Update payment status",
    description = "Updates the status of a payment. This endpoint is intended for internal use.",
    request_body = UpdatePaymentStatusByIdRequest,
    responses(
        (status = 200, description = "Payment status was successfully updated.", body = DocSuccessApiResponse<Payment>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the payment status update.", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Payment ID")
    ),
)]
#[patch("/{id}")]
async fn update_payment_status_by_id(
    db: Data<Database>,
    payment_id: Path<String>,
    payment_status: Json<UpdatePaymentStatusByIdRequest>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(jwt_payload) => jwt_payload,
        Err(err_res) => return err_res,
    };

    let payment_id = payment_id.into_inner();
    let payment_status = payment_status.into_inner();

    match service::update_payment_status_by_id(&db, payment_id, payment_status).await {
        Ok(payment) => {
            let response: ApiResponse<Payment> =
                ApiResponse::success("Payment Status was successfully updated.", Some(payment));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the payment status updating.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================

#[utoipa::path(
    delete,
    path = "/api/payments/{id}",
    tag = "Internal Endpoints",
    summary = "Delete a payment",
    description = "Deletes a payment specified by its ID. This endpoint is intended for internal use.",
    responses(
        (status = 200, description = "Payment was successfully deleted.", body = DocSuccessApiResponse<Payment>),
        (status = 401, description = "Error: Unauthorized", body = DocErrorApiResponse),
        (status = 500, description = "An error occurred during the payment deletion.", body = DocErrorApiResponse)
    ),
    params(
        ("id" = String, Path, description = "Payment ID")
    )
)]
#[delete("/{id}")]
async fn delete_payment_by_id(
    db: Data<Database>,
    payment_id: Path<String>,
    req: HttpRequest,
) -> impl Responder {
    match authenticate_internal_request(&req) {
        Ok(jwt_payload) => jwt_payload,
        Err(err_res) => return err_res,
    };

    let payment_id = payment_id.into_inner();

    match service::delete_payment_by_id(&db, payment_id).await {
        Ok(payment) => {
            let response: ApiResponse<Payment> =
                ApiResponse::success("Payment was successfully deleted.", Some(payment));
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                "An error occured during the payment deleting.",
                e.to_string(),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// =============================================================================================================================
