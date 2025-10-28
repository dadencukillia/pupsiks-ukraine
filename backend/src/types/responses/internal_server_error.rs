use serde::{Serialize, Deserialize};

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct InternalServerErrorResponse {
    pub message: String,
}

impl InternalServerErrorResponse {
    pub fn new(what: &'static str) -> Self {
        Self { 
            message: Errors::InternalServer { what }.to_string(), 
        }
    }
}

