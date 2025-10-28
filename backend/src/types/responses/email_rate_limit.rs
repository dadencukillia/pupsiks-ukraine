use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct EmailRateLimitErrorResponse {
    pub message: String,
    pub timestamp: u32
}

impl EmailRateLimitErrorResponse {
    pub fn new(how_much: u32, timestamp: u32) -> Self {
        Self {
            message: Errors::EmailRateLimit { how_much, timestamp }.to_string(),
            timestamp
        }
    }
}
