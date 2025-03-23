use common::{
    models::PaymentCurrency,
    utils::utils::{
        deserialize_datetime_from_any, serialize_option_object_id_as_hex_string,
        validate_date_not_in_past,
    },
};
use mongodb::bson::{
    DateTime, oid::ObjectId, serde_helpers::serialize_bson_datetime_as_rfc3339_string,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Success,
    Failed,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,
    pub amount: u32,
    pub currency: PaymentCurrency,
    pub status: PaymentStatus,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub created_at: DateTime,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,

    #[serde(rename = "event_id")]
    pub event_id: ObjectId,

    #[serde(rename = "ticket_id")]
    pub ticket_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePaymentRequest {
    #[validate(length(min = 12, max = 19, message = "Invalid card number length"))]
    pub card_number: String,

    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_datetime_from_any"
    )]
    #[validate(custom(function = "validate_date_not_in_past"))]
    pub expiration_date: DateTime,

    #[validate(length(min = 3, max = 4, message = "Invalid CVV"))]
    pub cvv: String,

    #[validate(length(
        min = 2,
        max = 50,
        message = "Card holder name must be between 2 and 50 characters"
    ))]
    pub card_holder: String,

    #[validate(range(min = 1, message = "Amount must be at least 1"))]
    pub amount: u32,

    pub currency: PaymentCurrency,

    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "event_id")]
    pub event_id: String,

    #[serde(rename = "ticket_id")]
    pub ticket_id: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePaymentStatusByIdRequest {
    pub status: PaymentStatus,
}

// =============================================================================================================================
