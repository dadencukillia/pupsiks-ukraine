use serde::Deserialize;
use validator::Validate;

use crate::utils::code_validations::{validate_email_code, validate_email_token};

#[derive(Deserialize, Validate, Debug)]
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

fn smart_trim(input: &str) -> String {
    let trimmed_and_cleaned: String = input
        .trim()
        .chars()
        .filter(|c| !c.is_control())
        .collect();

    let mut result = String::with_capacity(trimmed_and_cleaned.len());
    let mut last_char_was_space = false;

    for c in trimmed_and_cleaned.chars() {
        if c.is_whitespace() {
            if !last_char_was_space {
                result.push(' ');
                last_char_was_space = true;
            }
        } else {
            result.push(c);
            last_char_was_space = false;
        }
    }

    result.trim().to_string()
}

impl CreateCertRequest {
    pub fn trim(&self) -> Self {
        Self {
            email: smart_trim(&self.email),
            title: smart_trim(&self.title),
            name: smart_trim(&self.name),
            code: smart_trim(&self.code),
            token: smart_trim(&self.token),
        }
    }
}
