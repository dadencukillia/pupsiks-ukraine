use std::{collections::HashMap, sync::Arc};

use actix_web::{web::{self, Json}, Error, HttpRequest};
use chrono::{Duration, Utc};
use fred::{prelude::{Client, KeysInterface, ListInterface, TransactionInterface}, types::{Expiration, SetOptions, Value}};
use sea_orm::DatabaseConnection;
use short_uuid::ShortUuid;

use crate::{configs, models::cert, types::{errors::Errors, redis::EmailTask, requests::ForgotCertRequest, responses::success::CertEmailResponse}, utils::log_error::ResultLogger};

#[actix_web::post("/cert/forgot")]
pub async fn forgot_cert_endpoint(
    request: HttpRequest,
    body: Result<web::Json<ForgotCertRequest>, Error>,
    redis: web::Data<Arc<Client>>,
    db: web::Data<DatabaseConnection>
) -> Result<web::Json<CertEmailResponse>, Errors> {
    let place_name = "POST /api/v1/cert/forgot";

    match body.log_with_place_on_error(place_name){
        Ok(unclear_body) => {
            let body = unclear_body.trim();

            let find_result = cert::Entity::find_by_email(body.email.to_string())
                .one(db.as_ref())
                .await
                .log_with_place_on_error(place_name)
                .map_err(|_| Errors::InternalServer { what: "DB" })?;

            if let Some(certificate) = find_result {
                let addr_limit_key = format!("forgot_rate_limit:email:{}", body.email.to_string());
                let user_ip = match request.connection_info().realip_remote_addr() {
                    Some(ip) => ip.to_string(),
                    None => {
                        return Err(Errors::InternalServer { what: "IP address" });
                    },
                };
                let ip_limit_key = format!("forgot_rate_limit:ip:{}", user_ip);

                // Checking rate limit by IP address (3 letters in 10 minutes)
                {
                    let result: i64 = redis
                        .get(&ip_limit_key)
                        .await
                        .unwrap_or(0);

                    if result > 3 {
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

                // Checking and updating rate limit by email address (1 letter in 2 days)
                {

                    let rate_result: Option<String> = redis
                        .set(&addr_limit_key, "1", Some(Expiration::EX(2 * 24 * 60 * 60)), Some(SetOptions::NX), false)
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

                // Updating rate limit by IP address (3 letters in 10 minutes)
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

                // Adding email task into queue to be processed by a SMTP service
                let mut replacements = HashMap::new();
                replacements.insert("CERTID".to_string(), ShortUuid::from_uuid(&certificate.id).to_string());

                redis.lpush::<i64, _, _>(configs::get_redis_mail_task_queue(), serde_json::to_string(&EmailTask {
                    email: body.email.clone(),
                    purpose: "forgot".to_string(),
                    replacements: replacements,
                }).unwrap())
                    .await
                    .log_with_place_on_error(place_name)
                    .map_err(|_| Errors::InternalServer { what: "broker" })?;

                Ok(web::Json(
                    CertEmailResponse::new(body.email)
                ))
            } else {
                Err(Errors::ResourceNotFound { what: "certificate linked with this email" })
            }
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
