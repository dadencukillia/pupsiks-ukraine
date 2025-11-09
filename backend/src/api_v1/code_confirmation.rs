use std::{collections::HashMap, sync::Arc};

use actix_web::{web, Error, HttpRequest, Responder};
use chrono::{Duration, Utc};
use fred::{prelude::{Client, FredResult, HashesInterface, KeysInterface, ListInterface, TransactionInterface}, types::{Expiration, SetOptions, Value}};
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use validator::Validate;

use crate::{configs, models::cert, types::{errors::Errors, redis::EmailTask, requests::{SendCodePurposes, SendCodeRequest}, responses::success::CodeSentResponse}, utils::{code_generator::{generate_code_token, generate_email_code}, log_error::ResultLogger, uuid::get_uuid}};

#[actix_web::post("/send_code")]
pub async fn send_code_endpoint(
    request: HttpRequest,
    body: Result<web::Json<SendCodeRequest>, Error>,
    redis: web::Data<Arc<Client>>,
    db: web::Data<DatabaseConnection>
) -> Result<web::Json<CodeSentResponse>, Errors> {
    let place_name = "POST /api/v1/send_code";

    match body.log_with_place_on_error(place_name) {
        Ok(body_unclear) => {
            let body = body_unclear.trim();

            // Checking if email address is correct
            if body.validate()
                .log_with_place_on_error(place_name)
                .is_err() {
                return Err(Errors::BadRequest { what_invalid: "email field value" });
            }

            // Check special cases
            match body.purpose {
                SendCodePurposes::ConfirmCreation => {
                    let cert_to_check = cert::Entity::find_by_email(body.email.to_string())
                        .one(db.as_ref())
                        .await
                        .log_with_place_on_error(place_name)
                        .map_err(|_| Errors::InternalServer { what: "DB" })?;

                    if cert_to_check.is_some() {
                        return Err(Errors::AlreadyExists { what: "certificate with this email" });
                    }
                },
                SendCodePurposes::ConfirmDeletion { ref id } => {
                    let id = if let Some(id) = get_uuid(id) {
                        id
                    } else {
                        return Err(Errors::BadRequest { what_invalid: "id field value" });
                    };

                    let cert_to_check = cert::Entity::find()
                        .filter(cert::Column::Email.eq(body.email.to_string()))
                        .filter(cert::Column::Id.eq(id))
                        .one(db.as_ref())
                        .await
                        .log_with_place_on_error(place_name)
                        .map_err(|_| Errors::InternalServer { what: "DB" })?;

                    if cert_to_check.is_none() {
                        return Err(Errors::InvalidEmail);
                    }
                }
            };

            // Assigning keys for Redis rate limits
            let addr_limit_key = format!("code_rate_limit:email:{}", body.email);
            let user_ip = match request.connection_info().realip_remote_addr() {
                Some(ip) => ip.to_string(),
                None => {
                    return Err(Errors::InternalServer { what: "IP address" });
                },
            };
            let ip_limit_key = format!("code_rate_limit:ip:{}", user_ip);

            // Checking rate limit by IP address
            {
                let result: i64 = redis
                    .get(&ip_limit_key)
                    .await
                    .unwrap_or(0);

                if result > 5 {
                    let exp_time: i64 = redis.pttl(&ip_limit_key)
                        .await
                        .log_with_place_on_error(place_name)
                        .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                    let timestamp = (Utc::now() + Duration::milliseconds(exp_time)).timestamp();

                    return Err(Errors::IPRateLimit { 
                        how_much: (exp_time as u32) / 1000, 
                        timestamp: timestamp as u64
                    })
                }
            }

            // Checking and updating rate limit by email address (1 code in 3 minute)
            {
                let rate_result: Option<String> = redis
                    .set(&addr_limit_key, "1", Some(Expiration::EX(3 * 60)), Some(SetOptions::NX), false)
                    .await
                    .log_with_place_on_error(place_name)
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                match rate_result {
                    Some(_) => {},
                    None => {
                        let exp_time: i64 = redis.pttl(&addr_limit_key)
                            .await
                            .log_with_place_on_error(place_name)
                            .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                        let timestamp = (Utc::now() + Duration::milliseconds(exp_time)).timestamp();

                        return Err(Errors::EmailRateLimit { 
                            how_much: (exp_time as u32) / 1000, 
                            timestamp: timestamp as u64
                        });
                    }
                };
            }

            // Updating rate limit by IP address (5 codes in 10 minutes)
            {
                let pipeline = redis.multi();

                let result: Value = pipeline
                    .incr(&ip_limit_key)
                    .await
                    .log_with_place_on_error(place_name)
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                if !result.is_queued() {
                    return Err(Errors::InternalServer { what: "cache storage" });
                }

                let result: Value = pipeline
                    .expire(&ip_limit_key, 10 * 60, None)
                    .await
                    .log_with_place_on_error(place_name)
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                if !result.is_queued() {
                    return Err(Errors::InternalServer { what: "cache storage" });
                }

                let _: (i64, u8) = pipeline.exec(true)
                    .await
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;
            }

            // Generating code and token
            let email_code = generate_email_code();
            let email_token = generate_code_token();

            // Saving email code and token into storage for a day
            let key = format!("confirm_code:{}", body.email);
            let value = format!("{}:{}:{}", email_token, email_code, body.purpose.to_string());

            let expire_time = Utc::now() + Duration::seconds(24 * 60 * 60);

            let _: Option<String> = redis
                .set(&key, &value, Some(Expiration::EXAT(expire_time.timestamp())), None, false)
                .await
                .log_with_place_on_error(place_name)
                .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

            // Adding email task into queue to be processed by a SMTP service
            let mut replacements = HashMap::new();
            replacements.insert("CERTCODE".to_string(), email_code);

            redis.lpush::<i64, _, _>(configs::get_redis_mail_task_queue(), serde_json::to_string(&EmailTask {
                email: body.email.clone(),
                purpose: body.purpose.to_string(),
                replacements: replacements,
            }).unwrap())
                .await
                .log_with_place_on_error(place_name)
                .map_err(|_| Errors::InternalServer { what: "broker" })?;

            // Success
            Ok(web::Json(
                CodeSentResponse::new(body.email.to_string(), email_token, expire_time.timestamp() as u64)
            ))
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
