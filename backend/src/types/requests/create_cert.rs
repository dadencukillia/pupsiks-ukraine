use serde::Deserialize;
use validator::Validate;

use crate::utils::code_validations::{validate_email_code, validate_email_token};

#[derive(Deserialize, Validate)]
pub struct CreateCertRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 5))]
    pub title: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(custom(function = "validate_email_code"))]
    pub code: String,
    #[validate(custom(function = "validate_email_token"))]
    pub token: String
}
