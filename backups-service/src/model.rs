use common::utils::utils::{
    deserialize_datetime_from_any, deserialize_option_datetime_from_any,
    serialize_option_datetime_as_rfc3339_string, serialize_option_object_id_as_hex_string,
};
use mongodb::bson::{
    DateTime, oid::ObjectId, serde_helpers::serialize_bson_datetime_as_rfc3339_string,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = "Pending")]
pub enum BackupStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = "Auth")]
pub enum BackupService {
    Auth,
    Events,
    Notifications,
    Payments,
    Tickets,
    Users,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Backup {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    #[schema(example = "63f7b1c0a1234567890abcdef", value_type = String)]
    pub id: Option<ObjectId>,

    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    #[schema(example = "2025-03-23T08:37:10.975Z", value_type = String)]
    pub created_at: DateTime,

    #[serde(
        deserialize_with = "deserialize_option_datetime_from_any",
        serialize_with = "serialize_option_datetime_as_rfc3339_string"
    )]
    #[schema(example = "2025-03-23T09:00:00.000Z", value_type = String)]
    pub finished_at: Option<DateTime>,

    #[schema(example = "Auth", value_type = String)]
    pub service_name: BackupService,

    #[schema(example = "Pending", value_type = String)]
    pub status: BackupStatus,

    #[schema(example = "[{\"key\": \"value\"}]", value_type = Vec<serde_json::Value>)]
    pub data: Option<Vec<serde_json::Value>>,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetLastBackupByServiceName {
    #[schema(example = "Payments")]
    pub service_name: BackupService,
}
// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateBackup {
    #[schema(example = "Payments")]
    pub service_name: BackupService,
}

// =============================================================================================================================
