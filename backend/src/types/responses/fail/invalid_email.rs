use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct InvalidEmailErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl InvalidEmailErrorResponse {
    pub fn new() -> Self {
        Self { 
            code_error: "invalid_email".to_string(),
            message: Errors::InvalidEmail.to_string(), 
        }
    }
}

