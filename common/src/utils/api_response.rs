use crate::utils::utils::deserialize_object_id;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        #[serde(default = "default_true")]
        success: bool,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<T>,
    },
    Error {
        #[serde(default = "default_false")]
        success: bool,
        message: String,
        error: String,
    },
}

// =============================================================================================================================

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

// =============================================================================================================================

impl<T> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: Option<T>) -> Self {
        ApiResponse::Success {
            success: default_true(),
            message: message.into(),
            data,
        }
    }

    pub fn error(message: impl Into<String>, error: impl Into<String>) -> Self {
        ApiResponse::Error {
            success: default_false(),
            message: message.into(),
            error: error.into(),
        }
    }
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ObjectIdToString {
    pub id: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectIdWrapper {
    #[serde(rename = "id", deserialize_with = "deserialize_object_id")]
    pub id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DocSuccessApiResponse<T> {
    #[schema(example = true)]
    pub success: bool,
    #[schema(example = "Some success message")]
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DocErrorApiResponse {
    #[schema(example = false)]
    pub success: bool,
    #[schema(example = "Some error message")]
    pub message: String,
    #[schema(example = "Detailed error info")]
    pub error: String,
}

// =============================================================================================================================
