use common::utils::utils::{
    LETTERS_REGEX, serialize_option_object_id_as_hex_string, trim_lowercase,
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    #[schema(example = "63f7b1c0a1234567890abcdef", value_type = String)]
    pub id: Option<ObjectId>,

    #[schema(example = "john", value_type = String)]
    pub first_name: String,

    #[schema(example = "doe", value_type = String)]
    pub last_name: String,

    #[schema(example = "john.doe@example.com", value_type = String)]
    pub email: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct GetUserIdByEmailRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(email(message = "Email must be valid"))]
    #[schema(example = "john.doe@example.com", value_type = String)]
    pub email: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 30,
        message = "First name must be between 2 and 30 characters"
    ))]
    #[validate(regex(
        path = "*LETTERS_REGEX",
        message = "First name contains invalid characters"
    ))]
    #[schema(example = "john", value_type = String)]
    pub first_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 30,
        message = "Last name must be between 2 and 30 characters"
    ))]
    #[validate(regex(
        path = "*LETTERS_REGEX",
        message = "Last name contains invalid characters"
    ))]
    #[schema(example = "doe", value_type = String)]
    pub last_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(email(message = "Email must be valid"))]
    #[schema(example = "john.doe@example.com", value_type = String)]
    pub email: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
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
}

// =============================================================================================================================
