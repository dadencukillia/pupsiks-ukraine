use std::{error::Error, sync::Arc};

use fred::prelude::{Client, KeysInterface};

use crate::utils::code_validations::{validate_email_code, validate_email_token};

pub enum VerificationResult {
    Ok { purpose: String },
    NotFound,
    InvalidToken,
    InvalidCode,
    UnknownError(Box<dyn Error>)
}

pub async fn verify_email_code(
    redis: Arc<Client>,
    email: &str, token: &str, code: &str
) -> VerificationResult {
    if validate_email_code(code).is_err() || validate_email_token(token).is_err() {
        return VerificationResult::InvalidCode;
    }

    let key = format!("confirm_code:{}", email);

    let redis_code_data: Option<String> = match redis
        .get(&key)
        .await {
        Ok(a) => a,
        Err(err) => {
            return VerificationResult::UnknownError(Box::new(err));
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
