#[derive(Debug, Clone)]
#[cfg_attr(feature = "for-server", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "for-agent", derive(miniserde::Deserialize, miniserde::Serialize))]
struct ErrorMessage {
    error_id: u32,
    error_message: String,
}

#[cfg(feature = "for-server")]
pub fn generate_error_message(error_id: u32, error_message: String) -> serde_json::Value {
    serde_json::json!(ErrorMessage {
        error_id,
        error_message,
    })
}