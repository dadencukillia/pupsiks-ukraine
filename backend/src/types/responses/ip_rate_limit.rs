use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct IPRateLimitErrorResponse {
    pub message: String,
    pub timestamp: u32
}

impl IPRateLimitErrorResponse {
    pub fn new(how_much: u32, timestamp: u32) -> Self {
        Self {
            message: Errors::IPRateLimit { how_much, timestamp }.to_string(),
            timestamp
        }
    }
}

