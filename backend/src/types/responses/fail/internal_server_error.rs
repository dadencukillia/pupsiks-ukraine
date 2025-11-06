use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct InternalServerErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl InternalServerErrorResponse {
    pub fn new(what: &'static str) -> Self {
        Self { 
            code_error: "internal_server_error".to_string(),
            message: Errors::InternalServer { what }.to_string(), 
        }
    }
}

