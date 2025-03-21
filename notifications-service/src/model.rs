use common::utils::utils::{
    deserialize_datetime_from_any, serialize_option_object_id_as_hex_string,
};
use mongodb::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationType {
    Email,
    Sms,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,

    pub message: String,
    pub notif_type: NotificationType,
    pub status: NotificationStatus,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub created_at: DateTime,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateNotification {
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateNotificationStatus {
    pub status: NotificationStatus,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserInternalResponse {
    #[serde(rename = "id", alias = "_id")]
    pub id: String,

    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// =============================================================================================================================
