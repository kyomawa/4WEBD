use mongodb::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::utils::{
    deserialize_datetime_from_any, serialize_option_object_id_as_hex_string,
};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AuthRole {
    User,
    EventCreator,
    Operator,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerNotificationType {
    Email,
    Sms,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerNotificationStatus {
    Pending,
    Sent,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentCurrency {
    Eur,
    Usd,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TriggerNotificationRequest {
    #[validate(length(
        min = 10,
        max = 100,
        message = "Message must be between 10 and 100 characters"
    ))]
    pub message: String,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerNotificationResponse {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,

    pub message: String,
    pub notif_type: TriggerNotificationType,
    pub status: TriggerNotificationStatus,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub created_at: DateTime,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,
}

// =============================================================================================================================
