use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct AlreadyExistsErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl AlreadyExistsErrorResponse {
    pub fn new(what: &'static str) -> Self {
        Self { 
            code_error: "already_exists".to_string(),
            message: Errors::AlreadyExists { what }.to_string(), 
        }
    }
}
