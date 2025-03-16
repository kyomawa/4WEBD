use common::{
    models::AuthRole,
    utils::utils::{serialize_option_object_id_as_hex_string, trim, trim_lowercase},
};
use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Auth {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,
    pub password: String,
    pub role: AuthRole,

    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 12,
        max = 32,
        message = "password must be between 12 and 32 characters"
    ))]
    pub password: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_passwords", skip_on_field_errors = false))]
pub struct CreateAuthRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 30,
        message = "First name must be between 2 and 30 characters"
    ))]
    pub first_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 30,
        message = "Last name must be between 2 and 30 characters"
    ))]
    pub last_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(email(message = "Email must be valid"))]
    pub email: String,

    #[serde(deserialize_with = "trim")]
    #[validate(length(
        min = 12,
        max = 32,
        message = "password must be between 12 and 32 characters"
    ))]
    pub password: String,

    #[serde(deserialize_with = "trim")]
    #[validate(length(
        min = 12,
        max = 32,
        message = "password must be between 12 and 32 characters"
    ))]
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
