use mongodb::bson::oid::ObjectId;
use serde::Serializer;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;

// =============================================================================================================================

pub fn serialize_option_object_id_as_hex_string<S>(
    id: &Option<ObjectId>,
    serializer: S
) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    match id {
        Some(oid) => serialize_object_id_as_hex_string(oid, serializer),
        None => serializer.serialize_none(),
    }
}

// =============================================================================================================================
