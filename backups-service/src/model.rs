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
pub enum BackupStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BackupService {
    Auth,
    Events,
    Notifications,
    Payments,
    Tickets,
    Users,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
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
    pub date: DateTime,

    pub service_name: BackupService,
    pub status: BackupStatus,
    pub data: Vec<serde_json::Value>,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLastBackupByServiceName {
    pub service_name: BackupService,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateBackup {
    #[serde(
        deserialize_with = "deserialize_datetime_from_any",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub date: DateTime,

    pub service_name: BackupService,

    pub status: BackupStatus,

    pub data: Vec<serde_json::Value>,
}

// =============================================================================================================================
