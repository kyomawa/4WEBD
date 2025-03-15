use serde::{ Deserialize, Serialize };

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        success: bool,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<T>,
    },
    Error {
        success: bool,
        message: String,
        error: String,
    },
}

// =============================================================================================================================
