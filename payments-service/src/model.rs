use common::utils::utils::{
    deserialize_datetime_from_any, serialize_option_object_id_as_hex_string,
};
use mongodb::bson::{
    DateTime, oid::ObjectId, serde_helpers::serialize_bson_datetime_as_rfc3339_string,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentCurrency {
    Eur,
    Usd,
}

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
    pub user_id: String,

    #[serde(rename = "event_id")]
    pub event_id: String,

    #[serde(rename = "ticket_id")]
    pub ticket_id: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePaymentRequest {
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
