use serde::Serialize;

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct InvalidRouteErrorResponse {
    pub code_error: String,
    pub message: String,
}

impl InvalidRouteErrorResponse {
    pub fn new(correct_route: &'static str) -> Self {
        Self {
            code_error: "invalid_route".to_string(),
            message: Errors::InvalidRoute { correct_route: correct_route }.to_string(),
        }
    }
}

