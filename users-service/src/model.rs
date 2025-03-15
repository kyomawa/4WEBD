use mongodb::bson::oid::ObjectId;
use serde::{ Deserialize, Serialize };
use common::utils::utils::serialize_option_object_id_as_hex_string;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "id",
        alias = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// =============================================================================================================================
