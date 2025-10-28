use serde::{Serialize, Deserialize};

use crate::types::errors::Errors;

#[derive(Serialize)]
pub struct PageNotFoundErrorResponse {
    pub message: String,
    pub endpoints: Option<Vec<String>>,
}

impl PageNotFoundErrorResponse {
    pub fn new(endpoints: Option<Vec<String>>) -> Self {
        Self { 
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
