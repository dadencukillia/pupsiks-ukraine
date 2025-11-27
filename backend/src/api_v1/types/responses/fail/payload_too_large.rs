use serde::Serialize;
use crate::api_v1::types::errors::Errors;

#[derive(Serialize)]
pub struct PayloadTooLargeErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl PayloadTooLargeErrorResponse {
    pub fn new(bytes_limit: usize) -> Self {
        Self { 
            code_error: "payload_too_large".to_string(),
            message: Errors::PayloadTooLarge { bytes_limit }.to_string(), 
        }
    }
}

