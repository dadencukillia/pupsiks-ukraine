use std::sync::Arc;

use actix_web::{web, Error, Responder};
use fred::prelude::Client;
use validator::Validate;

use crate::{types::{errors::Errors, requests::CreateCertRequest}, utils::email_code_verify::{verify_email_code, VerificationResult}};

#[actix_web::post("/cert")]
pub async fn create_cert_endpoint(
    body: Result<web::Json<CreateCertRequest>, Error>,
    redis: web::Data<Arc<Client>>
) -> Result<String, Errors> {
    match body {
        Ok(body) => {
            if body.validate().is_err() {
                return Err(Errors::BadRequest { what_invalid: "field values" });
            }

            let ver_res = verify_email_code(redis.as_ref().clone(), &body.email, &body.token, &body.code).await;

            match ver_res {
                VerificationResult::Ok { purpose } => Ok(purpose),
                VerificationResult::InvalidCode => Ok("Invalid code".to_string()),
                VerificationResult::AnotherToken => Ok("Another token".to_string()),
                VerificationResult::NotFound => Ok("Not found".to_string()),
                VerificationResult::UnknownError(e) => panic!("{}", e),
            }
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
