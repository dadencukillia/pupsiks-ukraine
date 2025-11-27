use serde::Serialize;
use crate::api_v1::types::errors::Errors;

#[derive(Serialize)]
pub struct EmailRateLimitErrorResponse {
    pub code_error: String,
    pub message: String,
    pub timestamp: u64
}

impl EmailRateLimitErrorResponse {
    pub fn new(how_much: u32, timestamp: u64) -> Self {
        Self {
            code_error: "email_rate_limit".to_string(),
            message: Errors::EmailRateLimit { how_much, timestamp }.to_string(),
            timestamp
        }
    }
}
