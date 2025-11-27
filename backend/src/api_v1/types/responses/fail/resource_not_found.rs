use serde::Serialize;
use crate::api_v1::types::errors::Errors;

#[derive(Serialize)]
pub struct ResourceNotFoundErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl ResourceNotFoundErrorResponse {
    pub fn new(what: &'static str) -> Self {
        Self { 
            code_error: "resource_not_found".to_string(),
            message: Errors::ResourceNotFound { what }.to_string(), 
        }
    }
}
