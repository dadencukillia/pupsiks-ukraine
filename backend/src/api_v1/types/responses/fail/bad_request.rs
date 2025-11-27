use serde::Serialize;
use crate::api_v1::types::errors::Errors;

#[derive(Serialize)]
pub struct BadRequestErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl BadRequestErrorResponse {
    pub fn new(what_invalid: &'static str) -> Self {
        Self {
            code_error: "bad_request".to_string(),
            message: Errors::BadRequest { what_invalid: what_invalid }.to_string()
        }
    }
}
