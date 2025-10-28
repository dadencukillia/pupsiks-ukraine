use serde::{Deserialize, Serialize};
use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct BadRequestErrorResponse {
    pub message: String,
}

impl BadRequestErrorResponse {
    pub fn new(what_invalid: &'static str) -> Self {
        Self {
            message: Errors::BadRequest { what_invalid: what_invalid }.to_string()
        }
    }
}
