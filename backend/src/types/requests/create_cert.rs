use serde::Deserialize;
use validator::Validate;

use crate::utils::{code_validations::{validate_email_code, validate_email_token}, smart_trim::smart_trim};

#[derive(Deserialize, Validate, Debug)]
pub struct CreateCertRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 5, max = 100))]
    pub title: String,
    #[validate(custom(function = "validate_email_code"))]
    pub code: String,
    #[validate(custom(function = "validate_email_token"))]
    pub token: String
}

impl CreateCertRequest {
    pub fn trim(&self) -> Self {
        Self {
            email: smart_trim(&self.email),
            name: smart_trim(&self.name),
            title: smart_trim(&self.title),
            code: smart_trim(&self.code),
            token: smart_trim(&self.token),
        }
    }
}
