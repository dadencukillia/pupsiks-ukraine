use serde::Serialize;
use crate::api_v1::types::errors::Errors;

#[derive(Serialize)]
pub struct RequestsRateLimitErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl RequestsRateLimitErrorResponse {
    pub fn new() -> Self {
        Self { 
            code_error: "requests_rate_limit".to_string(),
            message: Errors::RequestsRateLimit.to_string(), 
        }
    }
}

