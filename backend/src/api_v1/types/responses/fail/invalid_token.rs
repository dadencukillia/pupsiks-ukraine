use serde::Serialize;
use crate::api_v1::types::errors::Errors;

#[derive(Serialize)]
pub struct InvalidTokenErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl InvalidTokenErrorResponse {
    pub fn new() -> Self {
        Self { 
            code_error: "invalid_token".to_string(),
            message: Errors::InvalidToken.to_string(), 
        }
    }
}

