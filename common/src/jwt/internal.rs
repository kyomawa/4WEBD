use std::env;

use actix_web::{ http::header, HttpRequest, HttpResponse };
use serde::{ Deserialize, Serialize };
use jsonwebtoken::{ decode, DecodingKey, Validation };
use once_cell::sync::Lazy;

use crate::utils::api_response::ApiResponse;

// =============================================================================================================================

pub static JWT_INTERNAL_SIGNATURE: Lazy<Vec<u8>> = Lazy::new(|| {
    let secret_str = env::var("JWT_INTERNAL_SIGNATURE").expect("JWT_INTERNAL_SIGNATURE not set");
    secret_str.into_bytes()
});

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalClaims {
    internal: bool,
    exp: i64,
}

pub fn decode_internal_jwt(token: &str) -> Result<InternalClaims, String> {
    let signature = JWT_INTERNAL_SIGNATURE.as_slice();
    decode::<InternalClaims>(token, &DecodingKey::from_secret(signature), &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| e.to_string())
}

pub fn get_internal_jwt(req: &HttpRequest) -> Result<InternalClaims, String> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or("Missing Authorization header")?;

    let auth_str = auth_header.to_str().map_err(|_| "Invalid header string")?;
    let token = auth_str.strip_prefix("Bearer ").ok_or("Invalid token format, expected Bearer")?;

    decode_internal_jwt(token)
}

pub fn authenticate_internal_request(req: &HttpRequest) -> Result<InternalClaims, HttpResponse> {
    match get_internal_jwt(req) {
        Ok(user) => Ok(user),
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::Error {
                success: false,
                message: "Une erreur est survenue !".to_string(),
                error: e,
            };
            Err(HttpResponse::Unauthorized().json(response))
        }
    }
}
// =============================================================================================================================
