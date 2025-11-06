use std::sync::Arc;
use actix_web::{web, Error};
use chrono::{Duration, Utc};
use fred::{prelude::{Client, KeysInterface, TransactionInterface}, types::{Expiration, Value}};
use sea_orm::{prelude::Uuid, ActiveValue::Set, DatabaseConnection, EntityTrait, SqlErr};
use validator::Validate;

use crate::{models::cert, types::{errors::Errors, requests::CreateCertRequest, responses::success::CertificateResponse}, utils::{email_code_verify::{verify_email_code, VerificationResult}, log_error::ResultLogger}};

#[actix_web::post("/cert")]
pub async fn create_cert_endpoint(
    body: Result<web::Json<CreateCertRequest>, Error>,
    redis: web::Data<Arc<Client>>,
    db: web::Data<DatabaseConnection>
) -> Result<web::Json<CertificateResponse>, Errors> {
    let place_name = "POST /api/v1/cert";

    match body.log_with_place_on_error(place_name) {
        Ok(body_unclear) => {
            // Cleaning and validating body
            let body = body_unclear.trim();
            println!("{:?}", body);

            if body
                .validate()
                .log_with_place_on_error(place_name)
                .is_err() {
                return Err(Errors::BadRequest { what_invalid: "field values" });
            }

            // Veryfing code
            let verification_result = verify_email_code(redis.as_ref().clone(), &body.email, &body.token, &body.code).await;

            match verification_result {
                VerificationResult::Ok { purpose } => {
                    if purpose != "create" {
                        // When created code had another purpose
                        return Err(Errors::InvalidRoute { correct_route: "DELETE /api/v1/cert" });
                    }

                    // Deleting code from Redis
                    let key = format!("confirm_code:{}", body.email);

                    let _: u8 = redis
                        .del(key)
                        .await
                        .log_with_place_on_error(place_name)
                        .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                    // Deleting code rate limit on email from Redis
                    let addr_limit_key = format!("code_rate_limit:email:{}", body.email);
                    let _: u8 = redis
                        .del(&addr_limit_key)
                        .await
                        .log_with_place_on_error(place_name)
                        .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                    // Creating and saving certificate into DB
                    let cert_to_insert = cert::ActiveModel {
                        email: Set(body.email.clone()),
                        name: Set(body.name.clone()),
                        title: Set(body.title.clone()),
                        ..Default::default()
                    };

                    let insert_result = match cert::Entity::insert(cert_to_insert)
                        .exec(db.as_ref())
                        .await
                        .log_with_place_on_error(place_name) {
                        Ok(a) => a,
                        Err(e) => {
                            if let Some(SqlErr::UniqueConstraintViolation(_)) = e.sql_err() {
                                return Err(Errors::AlreadyExists { what: "certificate with this email" });
                            } else {
                                return Err(Errors::InternalServer { what: "DB" });
                            }

                        },
                    };

                    let cert_uuid: Uuid = insert_result.last_insert_id;

                    // Updating stats
                    let _: Result<u64, _> = redis
                        .incr("stats:users_count")
                        .await
                        .log_on_error();

                    // Returning certificate data
                    Ok(web::Json(CertificateResponse::new(
                        &cert_uuid, 
                        body.name.to_string(), 
                        body.title.to_string()
                    )))
                },
                VerificationResult::InvalidToken => Err(Errors::InvalidToken),
                VerificationResult::NotFound => Err(Errors::ResourceNotFound { what: "code record" }),
                VerificationResult::UnknownError(e) => {
                    let _ = Err::<(), _>(e).log_with_place_on_error(place_name);
                    Err(Errors::InternalServer { what: "code verification" })
                },
                VerificationResult::InvalidCode => {
                    let key = format!("token_tries:{}", body.token);

                    // Increasing tries counter
                    let pipeline = redis.multi();

                    let result: Value = pipeline
                        .incr(&key)
                        .await
                        .log_with_place_on_error(place_name)
                        .map_err(|_| Errors::InvalidCode)?;

                    if !result.is_queued() {
                        return Err(Errors::InvalidCode);
                    }

                    let result: Value = pipeline
                        .expire(&key, 24 * 60 * 60, None)
                        .await
                        .log_with_place_on_error(place_name)
                        .map_err(|_| Errors::InvalidCode)?;

                    if !result.is_queued() {
                        return Err(Errors::InvalidCode);
                    }

                    let tries: (i64, u8) = pipeline.exec(true)
                        .await
                        .log_with_place_on_error(place_name)
                        .map_err(|_| Errors::InvalidCode)?;

                    // When tries out
                    if tries.0 >= 5 {
                        let addr_limit_key = format!("code_rate_limit:email:{}", body.email);
                        let code_confirm_key = format!("confirmcode:{}", body.email);

                        let how_much: i64 = 15 * 60; // 15 minutes
                        let timestamp = (Utc::now() + Duration::seconds(how_much)).timestamp();

                        // Removing activation code from storage
                        let _: Result<u8, _> = redis
                            .del(&code_confirm_key)
                            .await
                            .log_with_place_on_error(place_name);

                        // Removing tries counter from storage
                        let _: Result<u8, _> = redis
                            .del(&key)
                            .await
                            .log_with_place_on_error(place_name);

                        // Blocking email
                        let _: Result<Option<String>, _> = redis
                            .set(&addr_limit_key, "1", Some(Expiration::EX(how_much)), None, false)
                            .await
                            .log_with_place_on_error(place_name);

                        return Err(Errors::TriesOut { 
                            how_much: how_much as u32, 
                            timestamp: timestamp as u64
                        });
                    }

                    // When tries don't out
                    Err(Errors::InvalidCode)
                },
            }
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
