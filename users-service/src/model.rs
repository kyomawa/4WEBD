use mongodb::bson::{ oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string };
use serde::{ Deserialize, Serialize };

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id", alias = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// =============================================================================================================================
