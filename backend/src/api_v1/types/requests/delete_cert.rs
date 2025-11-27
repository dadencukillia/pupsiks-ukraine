use serde::Deserialize;
use validator::Validate;
use crate::{
    utils::smart_trim::smart_trim,
    api_v1::services::codes::{
        validate_email_code, 
        validate_email_token
    }
};

#[derive(Deserialize, Validate, Debug)]
pub struct DeleteCertRequest {
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_email_code"))]
    pub code: String,
    #[validate(custom(function = "validate_email_token"))]
    pub token: String
}

impl DeleteCertRequest {
    pub fn trim(&self) -> Self {
        Self {
            email: smart_trim(&self.email),
            code: smart_trim(&self.code),
            token: smart_trim(&self.token),
        }
    }
}
