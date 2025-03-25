use common::utils::utils::{
    deserialize_datetime_from_any, serialize_option_object_id_as_hex_string,
};
use mongodb::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = "Email")]
pub enum NotificationType {
    Email,
    Sms,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = "Pending")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Notification {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    #[schema(example = "63f7b1c0a1234567890abcdef", value_type = String)]
    pub id: Option<ObjectId>,

    #[schema(example = "Your event starts in 30 minutes.", value_type = String)]
    pub message: String,

    #[schema(example = "Email", value_type = String)]
    pub notif_type: NotificationType,

    #[schema(example = "Pending", value_type = String)]
    pub status: NotificationStatus,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    #[schema(example = "2025-03-23T08:37:10.975Z", value_type = String)]
    pub created_at: DateTime,

    #[serde(rename = "user_id")]
    #[schema(example = "63d88106c3f7903ba0f9211a", value_type = String)]
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateNotification {
    #[validate(length(
        min = 10,
        max = 100,
        message = "Message must be between 10 and 100 characters"
    ))]
    #[schema(example = "Your event is about to start. Please be ready.", value_type = String)]
    pub message: String,

    #[serde(rename = "user_id")]
    #[schema(example = "63d88106c3f7903ba0f9211a", value_type = String)]
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateNotificationStatus {
    #[schema(example = "Sent", value_type = String)]
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
