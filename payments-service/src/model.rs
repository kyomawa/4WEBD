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
use utoipa::ToSchema;
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = "Pending")]
pub enum PaymentStatus {
    Pending,
    Success,
    Failed,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Payment {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    #[schema(example = "63f7b1c0a1234567890abcdef", value_type = String)]
    pub id: Option<ObjectId>,

    #[schema(example = 150)]
    pub amount: u32,

    #[schema(example = "USD", value_type = String)]
    pub currency: PaymentCurrency,

    #[schema(example = "Pending")]
    pub status: PaymentStatus,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    #[schema(example = "2025-03-23T08:37:10.975Z", value_type = String)]
    pub created_at: DateTime,

    #[serde(rename = "user_id")]
    #[schema(example = "63d88106c3f7903ba0f9211a", value_type = String)]
    pub user_id: ObjectId,

    #[serde(rename = "event_id")]
    #[schema(example = "63da941412d5bd6dbc358950", value_type = String)]
    pub event_id: ObjectId,

    #[serde(rename = "ticket_id")]
    #[schema(example = "63daf9aefc24646c8d3fb79e", value_type = String)]
    pub ticket_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePaymentRequest {
    #[validate(length(min = 12, max = 19, message = "Invalid card number length"))]
    #[schema(example = "120156526352326230", value_type = String)]
    pub card_number: String,

    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_datetime_from_any"
    )]
    #[validate(custom(function = "validate_date_not_in_past"))]
    #[schema(example = "2027-04-12T00:00:00Z", value_type = String)]
    pub expiration_date: DateTime,

    #[validate(length(min = 3, max = 4, message = "Invalid CVV"))]
    #[schema(example = "373", value_type = String)]
    pub cvv: String,

    #[validate(length(
        min = 2,
        max = 50,
        message = "Card holder name must be between 2 and 50 characters"
    ))]
    #[schema(example = "Bryan Cellier", value_type = String)]
    pub card_holder: String,

    #[validate(range(min = 1, message = "Amount must be at least 1"))]
    #[schema(example = 150)]
    pub amount: u32,

    #[schema(example = "USD", value_type = String)]
    pub currency: PaymentCurrency,

    #[serde(rename = "user_id")]
    #[schema(example = "63d88106c3f7903ba0f9211a", value_type = String)]
    pub user_id: String,

    #[serde(rename = "event_id")]
    #[schema(example = "63da941412d5bd6dbc358950", value_type = String)]
    pub event_id: String,

    #[serde(rename = "ticket_id")]
    #[schema(example = "63daf9aefc24646c8d3fb79e", value_type = String)]
    pub ticket_id: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePaymentStatusByIdRequest {
    #[schema(example = "Success")]
    pub status: PaymentStatus,
}

// =============================================================================================================================
