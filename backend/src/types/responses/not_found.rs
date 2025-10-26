use serde::{Serialize, Deserialize};

use crate::types::errors::Errors;

#[derive(Serialize, Deserialize)]
pub struct NotFoundErrorResponse {
    pub message: String,
    pub endpoints: Option<Vec<String>>,
}

impl NotFoundErrorResponse {
    pub fn new(endpoints: Option<Vec<String>>) -> Self {
        NotFoundErrorResponse { 
            message: Errors::NotFound { endpoints: None }.to_string(), 
            endpoints: endpoints
        }
    }
}

impl From<&'static [(&'static str, &'static str)]> for NotFoundErrorResponse {
    fn from(value: &'static [(&'static str, &'static str)]) -> Self {
        Self::new(Some(
            value
                .iter()
                .map(|(method, path)| format!("{} {}", method, path))
                .collect::<Vec<String>>()
        ))
    }
}

impl From<Option<&'static [(&'static str, &'static str)]>> for NotFoundErrorResponse {
    fn from(value: Option<&'static [(&'static str, &'static str)]>) -> Self {
        if let Some(a) = value {
            Self::from(a)
        } else {
            Self::new(None)
        }
    }
}

