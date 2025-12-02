use chrono::{DateTime, Duration, Utc};
use rand::prelude::*;
use rand::rngs::OsRng;
use validator::ValidationError;
use crate::api_v1::repos::RedisRepo;
use anyhow::{Result, Error};

pub const EMAIL_CODE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const EMAIL_CODE_NUMBERS: &[u8] = b"0123456789";
pub const EMAIL_TOKEN_SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Generates a random code in the 3 LETTERS + 3 NUMBERS + 3 LETTERS format
pub fn generate_email_code() -> String {
    let mut rng = OsRng;

    let mut gen_from_dict = |dict: &[u8], length: usize| -> String {
        (0..length).map(|_| {
            let random_index = rng.gen_range(0..dict.len());
            dict[random_index] as char
        }).collect()
    };

    format!("{}{}{}", 
        gen_from_dict(EMAIL_CODE_LETTERS, 3),
        gen_from_dict(EMAIL_CODE_NUMBERS, 3),
        gen_from_dict(EMAIL_CODE_LETTERS, 3),
    )
}

/// Generates a random token in the 32 LETTERS format
pub fn generate_code_token() -> String {
    let mut rng = OsRng;

    (0..32).map(|_| {
        let random_index = rng.gen_range(0..EMAIL_TOKEN_SYMBOLS.len());
        EMAIL_TOKEN_SYMBOLS[random_index] as char
    }).collect()
}

/// Checks if the code in the correct format
pub fn validate_email_code(code: &str) -> Result<(), ValidationError> {
    if code.len() != 9 {
        return Err(ValidationError::new("too_short_email_code"));
    }

    let mut chars = code.chars().map(|c| c.to_uppercase().next().unwrap_or(' '));

    for _ in 0..3 {
        let code_char = chars.next().unwrap();
        if !EMAIL_CODE_LETTERS.iter().any(|a| *a as char == code_char) {
            return Err(ValidationError::new("first_three_characters_must_be_letters"));
        }
    }

    for _ in 0..3 {
        let code_char = chars.next().unwrap();
        if !EMAIL_CODE_NUMBERS.iter().any(|a| *a as char == code_char) {
            return Err(ValidationError::new("seconds_three_characters_must_be_numbers"));
        }
    }

    for _ in 0..3 {
        let code_char = chars.next().unwrap();
        if !EMAIL_CODE_LETTERS.iter().any(|a| *a as char == code_char) {
            return Err(ValidationError::new("third_three_characters_must_be_letters"));
        }
    }

    Ok(())
}

/// Checks if the token in the correct format
pub fn validate_email_token(token: &str) -> Result<(), ValidationError> {
    if token.len() != 32 {
        return Err(ValidationError::new("too_short_token"));
    }

    let chars = token.chars();

    for code_char in chars {
        if !EMAIL_TOKEN_SYMBOLS.iter().any(|a| *a as char == code_char) {
            return Err(ValidationError::new("invalid_characters_set"));
        }
    }

    Ok(())
}

pub enum VerificationResult {
    Ok { purpose: String },
    NotFound,
    InvalidToken,
    InvalidCode,
    UnknownError(Error)
}

/// Validates code and token, compares stored values with the user's ones
pub async fn verify_email_code(
    redis: &RedisRepo,
    email: &str, token: &str, code: &str
) -> VerificationResult {
    if validate_email_code(code).is_err() || validate_email_token(token).is_err() {
        return VerificationResult::InvalidCode;
    }

    let key = format!("confirm_code:{}", email);

    let redis_code_data = match redis.get_value::<String>(key).await {
        Ok(a) => a,
        Err(e) => {
            return VerificationResult::UnknownError(e)
        }
    };

    match redis_code_data {
        Some(code_string) => {
            let mut split = code_string.split(":");

            let redis_token = match split.next() {
                Some(a) => a,
                None => {
                    return VerificationResult::NotFound;
                }
            };

            let redis_code = match split.next() {
                Some(a) => a,
                None => {
                    return VerificationResult::NotFound;
                }
            };

            let redis_purpose = match split.next() {
                Some(a) => a,
                None => {
                    return VerificationResult::NotFound;
                }
            };

            if redis_token != token {
                return VerificationResult::InvalidToken;
            }

            if redis_code != code {
                return VerificationResult::InvalidCode;
            }

            VerificationResult::Ok {
                purpose: redis_purpose.to_string()
            }
        },
        None => VerificationResult::NotFound,
    }
}

/// Stores the code and all details about it in the storage to be ready for use for confirmation
pub async fn save_code_in_storage(
    redis: &RedisRepo,
    email: &str, purpose: &str, generated_code: &str, generated_token: &str
) -> Result<DateTime<Utc>> {
    let key = format!("confirm_code:{}", email);
    let value = format!("{}:{}:{}", generated_token, generated_code, purpose.to_string());

    let expire_time = Duration::seconds(24 * 60 * 60);

    let _ = redis
        .set_value(key, value, expire_time, true)
        .await?;

    Ok(Utc::now() + expire_time)
}

/// Removes the code from the storage to make it inaccessible for confirmation
pub async fn remove_code_from_storage(
    redis: &RedisRepo,
    email: &str
) -> Result<()> {
    let key = format!("confirm_code:{}", email);

    redis
        .delete_by_key(key)
        .await?;

    Ok(())
}
