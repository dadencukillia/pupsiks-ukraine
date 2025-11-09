use serde::Deserialize;
use validator::Validate;

use crate::utils::{code_validations::{validate_email_code, validate_email_token}, smart_trim::smart_trim};

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
