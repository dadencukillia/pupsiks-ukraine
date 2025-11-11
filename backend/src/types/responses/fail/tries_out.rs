use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct TriesOutErrorResponse {
    pub code_error: String,
    pub message: String,
    pub timestamp: u64
}

impl TriesOutErrorResponse {
    pub fn new(how_much: u32, timestamp: u64) -> Self {
        Self {
            code_error: "tries_out".to_string(),
            message: Errors::TriesOut { how_much, timestamp }.to_string(),
            timestamp
        }
    }
}

