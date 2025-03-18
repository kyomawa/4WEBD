use common::utils::utils::{serialize_option_object_id_as_hex_string, trim};
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
    pub date: DateTime,
    pub capacity: u16,
    pub remaining_seats: u16,
    pub created_at: DateTime,

    #[serde(rename = "creator_id")]
    pub creator_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_create_event", skip_on_field_errors = false))]
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

    #[validate(custom(function = "validate_date_not_in_past"))]
    pub date: DateTime,

    #[validate(range(min = 25, message = "Capacity must be at least 25"))]
    pub capacity: u16,

    pub remaining_seats: u16,

    #[serde(rename = "creator_id")]
    pub creator_id: ObjectId,
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

    #[validate(custom(function = "validate_date_not_in_past"))]
    pub date: DateTime,

    #[validate(range(min = 25, message = "Capacity must be at least 25"))]
    pub capacity: u16,

    pub remaining_seats: u16,
}

// =============================================================================================================================

fn validate_date_not_in_past(date: &DateTime) -> Result<(), ValidationError> {
    let now = chrono::Utc::now();
    let event_date_chrono = date.to_chrono();

    if event_date_chrono < now {
        let mut err = ValidationError::new("date_in_past");
        err.message = Some("The event date cannot be in the past.".into());
        return Err(err);
    }

    Ok(())
}

fn validate_create_event(req: &CreateEventRequest) -> Result<(), ValidationError> {
    if req.remaining_seats > req.capacity {
        let mut err = ValidationError::new("remaining_seats_exceeds_capacity");
        err.message = Some("Remaining seats cannot exceed capacity.".into());
        return Err(err);
    }
    Ok(())
}

fn validate_update_event(req: &UpdateEventRequest) -> Result<(), ValidationError> {
    if req.remaining_seats > req.capacity {
        let mut err = ValidationError::new("remaining_seats_exceeds_capacity");
        err.message = Some("Remaining seats cannot exceed capacity.".into());
        return Err(err);
    }
    Ok(())
}

// =============================================================================================================================
