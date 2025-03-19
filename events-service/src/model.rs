use common::utils::utils::{
    deserialize_datetime_from_any, serialize_option_object_id_as_hex_string, trim,
    validate_date_not_in_past,
};
use mongodb::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,

    pub title: String,
    pub description: String,
    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub date: DateTime,
    pub capacity: u32,
    pub remaining_seats: u32,

    pub price: u32,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub created_at: DateTime,

    #[serde(rename = "creator_id")]
    pub creator_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateEventRequest {
    #[serde(deserialize_with = "trim")]
    #[validate(length(
        min = 2,
        max = 100,
        message = "Title must be between 2 and 100 characters"
    ))]
    pub title: String,

    #[serde(deserialize_with = "trim")]
    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,

    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_datetime_from_any"
    )]
    #[validate(custom(function = "validate_date_not_in_past"))]
    pub date: DateTime,

    #[validate(range(min = 25, message = "Capacity must be at least 25"))]
    pub capacity: u32,

    #[validate(range(min = 1, message = "Price must be atleast one."))]
    pub price: u32,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_update_event", skip_on_field_errors = false))]
pub struct UpdateEventRequest {
    #[serde(deserialize_with = "trim")]
    #[validate(length(
        min = 2,
        max = 100,
        message = "Title must be between 2 and 100 characters"
    ))]
    pub title: String,

    #[serde(deserialize_with = "trim")]
    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,

    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_datetime_from_any"
    )]
    #[validate(custom(function = "validate_date_not_in_past"))]
    pub date: DateTime,

    #[validate(range(min = 25, message = "Capacity must be at least 25"))]
    pub capacity: u32,

    pub remaining_seats: u32,

    #[validate(range(min = 1, message = "Price must be atleast one."))]
    pub price: u32,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSeatsRequest {
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
