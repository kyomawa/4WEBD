use common::utils::utils::{serialize_option_object_id_as_hex_string, trim_lowercase};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
enum AuthRole {
    User,
    EventCreator,
    Operator,
    Admin,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Auth {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    id: Option<ObjectId>,
    password: String,
    role: AuthRole,
    user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_passwords", skip_on_field_errors = false))]
pub struct CreateAuthRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 12,
        max = 32,
        message = "password must be between 12 and 32 characters"
    ))]
    password: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 12,
        max = 32,
        message = "password must be between 12 and 32 characters"
    ))]
    confirm_password: String,
    role: AuthRole,
    user_id: ObjectId,
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

    let pwned_check = futures::executor::block_on(async {
        let pwned = pwned::api::PwnedBuilder::default().build().unwrap();
        pwned.check_password(&req.password).await
    });

    match pwned_check {
        Ok(response) => {
            if response.found {
                let mut error = ValidationError::new("password_pwned");
                error.message = Some("This password has been pwned. Please choose another.".into());
                return Err(error);
            }
        }
        Err(e) => {
            let mut error = ValidationError::new("pwned_api_error");
            error.message = Some(format!("Error checking password pwned: {}", e).into());
            return Err(error);
        }
    }

    Ok(())
}

// =============================================================================================================================
