use serde::{Serialize, Deserialize};

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct PageNotFoundErrorResponse {
    pub code_error: String,
    pub message: String,
    pub endpoints: Option<Vec<String>>,
}

impl PageNotFoundErrorResponse {
    pub fn new(endpoints: Option<Vec<String>>) -> Self {
        Self { 
            code_error: "page_not_found".to_string(),
            message: Errors::PageNotFound { endpoints: None }.to_string(), 
            endpoints: endpoints
        }
    }

    pub fn endpoints_to_vec(value: &'static [(&'static str, &'static str)]) -> Vec<String> {
        value
            .iter()
            .map(|(method, path)| format!("{} {}", method, path))
            .collect::<Vec<String>>()
    }
}
