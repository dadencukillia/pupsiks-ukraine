use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct InvalidCodeErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl InvalidCodeErrorResponse {
    pub fn new() -> Self {
        Self { 
            code_error: "invalid_code".to_string(),
            message: Errors::InvalidCode.to_string(), 
        }
    }
}

