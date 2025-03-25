use common::{
    models::AuthRole,
    utils::utils::{serialize_option_object_id_as_hex_string, trim, trim_lowercase},
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct Auth {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    #[schema(example = "63f7b1c0a1234567890abcdef", value_type = String)]
    pub id: Option<ObjectId>,

    #[schema(example = "password_hashed", value_type = String)]
    pub password: String,

    #[schema(example = "User", value_type = String)]
    pub role: AuthRole,

    #[serde(rename = "user_id")]
    #[schema(example = "63d88106c3f7903ba0f9211a", value_type = String)]
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(email(message = "Email must be valid"))]
    #[schema(example = "john.doe@example.com", value_type = String)]
    pub email: String,

    #[validate(length(
        min = 2,
        max = 64,
        message = "password must be between 2 and 64 characters"
    ))]
    #[schema(example = "SecurePass123", value_type = String)]
    pub password: String,
}

// =============================================================================================================================

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({ "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiNjNmN2IxYzBhMTIzNDU2Nzg5MGFiY2RlZiIsInJvbGUiOiJVc2VyIiwiZXhwIjoyMzQ3MDQyNzk2fQ.xu9lefEr9gP0HOUzTIYuEcb4oDHViaO72GFtKuF9gmw" }))]
pub struct LoginResponse {
    pub token: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserInternalResponse {
    #[serde(rename = "id", alias = "_id")]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[validate(schema(function = "validate_passwords", skip_on_field_errors = false))]
pub struct CreateAuthRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 30,
        message = "First name must be between 2 and 30 characters"
    ))]
    #[schema(example = "john", value_type = String)]
    pub first_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 30,
        message = "Last name must be between 2 and 30 characters"
    ))]
    #[schema(example = "doe", value_type = String)]
    pub last_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(email(message = "Email must be valid"))]
    #[schema(example = "john.doe@example.com", value_type = String)]
    pub email: String,

    #[serde(deserialize_with = "trim")]
    #[validate(length(
        min = 12,
        max = 32,
        message = "password must be between 12 and 32 characters"
    ))]
    #[schema(example = "SecurePass123!", value_type = String)]
    pub password: String,

    #[serde(deserialize_with = "trim")]
    #[validate(length(
        min = 12,
        max = 32,
        message = "password must be between 12 and 32 characters"
    ))]
    #[schema(example = "SecurePass123!", value_type = String)]
    pub confirm_password: String,
}

// =============================================================================================================================

fn validate_passwords(req: &CreateAuthRequest) -> Result<(), ValidationError> {
    if req.password != req.confirm_password {
        let mut error = ValidationError::new("password_mismatch");
        error.message = Some("password and confirm_password must match".into());
        return Err(error);
    }

    if !req.password.chars().any(|c| c.is_lowercase()) {
        let mut error = ValidationError::new("password_no_lowercase");
        error.message = Some("Password must contain at least one lowercase letter.".into());
        return Err(error);
    }

    if !req.password.chars().any(|c| c.is_uppercase()) {
        let mut error = ValidationError::new("password_no_uppercase");
        error.message = Some("Password must contain at least one uppercase letter.".into());
        return Err(error);
    }

    if !req.password.chars().any(|c| c.is_ascii_digit()) {
        let mut error = ValidationError::new("password_no_digit");
        error.message = Some("Password must contain at least one digit.".into());
        return Err(error);
    }

    if !req.password.chars().any(|c| !c.is_alphanumeric()) {
        let mut error = ValidationError::new("password_no_special");
        error.message = Some("Password must contain at least one special character.".into());
        return Err(error);
    }

    Ok(())
}

// =============================================================================================================================
