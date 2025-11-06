use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct IPRateLimitErrorResponse {
    pub code_error: String,
    pub message: String,
    pub timestamp: u64
}

impl IPRateLimitErrorResponse {
    pub fn new(how_much: u32, timestamp: u64) -> Self {
        Self {
            code_error: "ip_rate_limit".to_string(),
            message: Errors::IPRateLimit { how_much, timestamp }.to_string(),
            timestamp
        }
    }
}

