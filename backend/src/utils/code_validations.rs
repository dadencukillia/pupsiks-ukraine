use validator::ValidationError;

use crate::utils::code_generator::{EMAIL_CODE_LETTERS, EMAIL_CODE_NUMBERS, EMAIL_TOKEN_SYMBOLS};

pub fn validate_email_code(code: &str) -> Result<(), ValidationError> {
    if code.len() != 9 {
        return Err(ValidationError::new("too_small"));
    }

    let mut chars = code.chars().map(|c| c.to_uppercase().next().unwrap_or(' '));

    for _ in 0..3 {
        let code_char = chars.next().unwrap();
        if !EMAIL_CODE_LETTERS.iter().any(|a| *a as char == code_char) {
            return Err(ValidationError::new("first_three_are_not_letters"));
        }
    }

    for _ in 0..3 {
        let code_char = chars.next().unwrap();
        if !EMAIL_CODE_NUMBERS.iter().any(|a| *a as char == code_char) {
            return Err(ValidationError::new("second_three_are_not_numbers"));
        }
    }

    for _ in 0..3 {
        let code_char = chars.next().unwrap();
        if !EMAIL_CODE_LETTERS.iter().any(|a| *a as char == code_char) {
            return Err(ValidationError::new("third_three_are_not_letters"));
        }
    }

    Ok(())
}

pub fn validate_email_token(token: &str) -> Result<(), ValidationError> {
    if token.len() != 32 {
        return Err(ValidationError::new("too_small"));
    }

    let chars = token.chars();

    for code_char in chars {
        if !EMAIL_TOKEN_SYMBOLS.iter().any(|a| *a as char == code_char) {
            return Err(ValidationError::new("invalid_characters"));
        }
    }

    Ok(())
}
