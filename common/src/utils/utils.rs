use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::de::Deserializer;
use serde::{Deserialize, Serializer};

// =============================================================================================================================

pub fn serialize_option_object_id_as_hex_string<S>(
    id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(oid) => serialize_object_id_as_hex_string(oid, serializer),
        None => serializer.serialize_none(),
    }
}

// =============================================================================================================================

pub fn trim_lowercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_lowercase())
}

// =============================================================================================================================
