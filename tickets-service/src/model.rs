use common::utils::utils::{
    deserialize_datetime_from_any, serialize_option_object_id_as_hex_string,
};
use mongodb::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub enum TicketStatus {
    Active,
    Refunded,
    Cancelled,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub purchase_date: DateTime,

    pub seat_number: u32,
    pub status: TicketStatus,
    pub price: u32,

    #[serde(rename = "event_id")]
    pub event_id: ObjectId,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTicketRequest {
    #[validate(range(min = 1, message = "Seat number must be at least one."))]
    pub seat_number: u32,

    #[serde(rename = "event_id")]
    pub event_id: ObjectId,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTicketRequest {
    #[validate(range(min = 1, message = "Seat number must be at least one."))]
    pub seat_number: u32,

    pub status: Option<TicketStatus>,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEventInternalResponse {
    #[serde(rename = "id", alias = "_id")]
    pub id: String,

    pub title: String,
    pub description: String,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub date: DateTime,

    pub capacity: u32,
    pub remaining_seats: u32,
    pub location: String,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub created_at: DateTime,

    pub price: u32,

    #[serde(rename = "creator_id")]
    pub creator_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEventRemainingSeatsInternalResponse {
    #[serde(rename = "id", alias = "_id")]
    pub id: String,

    pub title: String,
    pub description: String,
    pub date: DateTime,
    pub capacity: u32,
    pub location: String,
    pub remaining_seats: u32,
    pub created_at: DateTime,
    pub price: u32,

    #[serde(rename = "creator_id")]
    pub creator_id: ObjectId,
}

// =============================================================================================================================
