use actix_web::{
    HttpResponse,
    error::InternalError,
    web::{self, JsonConfig},
};
use common::utils::api_response::ApiResponse;

// =============================================================================================================================

pub fn deserialize_error_extractor() -> JsonConfig {
    web::JsonConfig::default().error_handler(|err, _req| {
        let api_response = ApiResponse::<()>::error("Deserialization error", err.to_string());
        InternalError::from_response(err, HttpResponse::BadRequest().json(api_response)).into()
    })
}

// =============================================================================================================================
