use common::utils::utils::{
    deserialize_datetime_from_any, serialize_option_object_id_as_hex_string, trim,
    validate_date_not_in_past,
};
use mongodb::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Event {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    #[schema(example = "63f7b1c0a1234567890abcdef", value_type = String)]
    pub id: Option<ObjectId>,

    #[schema(example = "Music Festival", value_type = String)]
    pub title: String,

    #[schema(example = "A fun and exciting outdoor music festival.", value_type = String)]
    pub description: String,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    #[schema(example = "2025-08-15T18:00:00Z", value_type = String)]
    pub date: DateTime,

    #[schema(example = 500)]
    pub capacity: u32,

    #[schema(example = "Central Park", value_type = String)]
    pub location: String,

    #[schema(example = 450)]
    pub remaining_seats: u32,

    #[schema(example = 75)]
    pub price: u32,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    #[schema(example = "2023-06-01T12:00:00Z", value_type = String)]
    pub created_at: DateTime,

    #[serde(rename = "creator_id")]
    #[schema(example = "63d88106c3f7903ba0f9211a", value_type = String)]
    pub creator_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateEventRequest {
    #[serde(deserialize_with = "trim")]
    #[schema(example = "Music Festival", value_type = String)]
    #[validate(length(
        min = 2,
        max = 100,
        message = "Title must be between 2 and 100 characters"
    ))]
    pub title: String,

    #[serde(deserialize_with = "trim")]
    #[schema(example = "A fun and exciting outdoor music festival.", value_type = String)]
    #[validate(length(
        min = 10,
        max = 500,
        message = "Description must be between 10 and 500 characters"
    ))]
    pub description: String,

    #[serde(deserialize_with = "trim")]
    #[schema(example = "Central Park", value_type = String)]
    #[validate(length(
        min = 2,
        max = 500,
        message = "Location must be between 2 and 75 characters"
    ))]
    pub location: String,

    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_datetime_from_any"
    )]
    #[schema(example = "2025-08-15T18:00:00Z", value_type = String)]
    #[validate(custom(function = "validate_date_not_in_past"))]
    pub date: DateTime,

    #[schema(example = 500)]
    #[validate(range(min = 25, message = "Capacity must be at least 25"))]
    pub capacity: u32,

    #[schema(example = 75)]
    #[validate(range(min = 1, message = "Price must be at least one."))]
    pub price: u32,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[validate(schema(function = "validate_update_event", skip_on_field_errors = false))]
pub struct UpdateEventRequest {
    #[serde(deserialize_with = "trim")]
    #[schema(example = "Updated Music Festival", value_type = String)]
    #[validate(length(
        min = 2,
        max = 100,
        message = "Title must be between 2 and 100 characters"
    ))]
    pub title: String,

    #[serde(deserialize_with = "trim")]
    #[schema(example = "An updated description of the music festival.", value_type = String)]
    #[validate(length(
        min = 10,
        max = 500,
        message = "Description must be between 10 and 500 characters"
    ))]
    pub description: String,

    #[serde(deserialize_with = "trim")]
    #[schema(example = "Downtown Arena", value_type = String)]
    #[validate(length(
        min = 2,
        max = 500,
        message = "Location must be between 2 and 75 characters"
    ))]
    pub location: String,

    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_datetime_from_any"
    )]
    #[schema(example = "2025-09-01T20:00:00Z", value_type = String)]
    #[validate(custom(function = "validate_date_not_in_past"))]
    pub date: DateTime,

    #[schema(example = 600)]
    #[validate(range(min = 25, message = "Capacity must be at least 25"))]
    pub capacity: u32,

    #[schema(example = 550)]
    pub remaining_seats: u32,

    #[schema(example = 85)]
    #[validate(range(min = 1, message = "Price must be at least one."))]
    pub price: u32,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateSeatsRequest {
    #[schema(example = -10)]
    pub delta: i32,
}

// =============================================================================================================================

fn validate_update_event(req: &UpdateEventRequest) -> Result<(), ValidationError> {
    if req.remaining_seats > req.capacity {
        let mut err = ValidationError::new("remaining_seats_exceeds_capacity");
        err.message = Some("Remaining seats cannot exceed capacity.".into());
        return Err(err);
    }
    Ok(())
}

// =============================================================================================================================
