use std::env;

use actix_web::{HttpRequest, HttpResponse, http::header};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::utils::api_response::ApiResponse;

// =============================================================================================================================

pub static JWT_INTERNAL_SIGNATURE: Lazy<Vec<u8>> = Lazy::new(|| {
    let secret_str = env::var("JWT_INTERNAL_SIGNATURE").expect("JWT_INTERNAL_SIGNATURE not set");
    secret_str.into_bytes()
});

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalClaims {
    pub internal: bool,
    pub exp: i64,
}

// =============================================================================================================================

pub fn encode_internal_jwt() -> Result<String, String> {
    let claims = InternalClaims {
        internal: true,
        exp: (Utc::now() + Duration::minutes(5)).timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_INTERNAL_SIGNATURE.as_slice()),
    )
    .map_err(|e| e.to_string())
}

// =============================================================================================================================

pub fn decode_internal_jwt(token: &str) -> Result<InternalClaims, String> {
    let signature = JWT_INTERNAL_SIGNATURE.as_slice();
    decode::<InternalClaims>(
        token,
        &DecodingKey::from_secret(signature),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}

// =============================================================================================================================

pub fn get_internal_jwt(req: &HttpRequest) -> Result<InternalClaims, String> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or("Missing Authorization header")?;

    let auth_str = auth_header.to_str().map_err(|_| "Invalid header string")?;
    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or("Invalid token format, expected Bearer")?;

    decode_internal_jwt(token)
}

// =============================================================================================================================

pub fn authenticate_internal_request(req: &HttpRequest) -> Result<InternalClaims, HttpResponse> {
    match get_internal_jwt(req) {
        Ok(payload) => Ok(payload),
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error("The request to this endpoint must be internal.", e);
            Err(HttpResponse::Unauthorized().json(response))
        }
    }
}
// =============================================================================================================================
