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
    utils::api_response::ApiResponse,
};
use mongodb::Database;

use crate::{
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
        .service(delete_payment_by_id);

    cfg.service(scope);
}

// =============================================================================================================================

#[get("/health")]
async fn health_check() -> impl Responder {
    let response: ApiResponse<()> = ApiResponse::success("🟢 Payments Service is Alive.", None);
    HttpResponse::Ok().json(response)
}

// =============================================================================================================================

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
