use serde::{Serialize, Deserialize};

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct ResourceNotFoundErrorResponse {
    pub message: String,
}

impl ResourceNotFoundErrorResponse {
    pub fn new(what: &'static str) -> Self {
        Self { 
            message: Errors::ResourceNotFound { what }.to_string(), 
        }
    }
}
