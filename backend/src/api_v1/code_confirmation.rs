use std::{collections::HashMap, sync::Arc};

use actix_web::{web, Error, HttpRequest, Responder};
use chrono::{Duration, Utc};
use fred::{prelude::{Client, FredResult, HashesInterface, KeysInterface, ListInterface}, types::{Expiration, SetOptions}};
use validator::Validate;

use crate::{configs, types::{errors::Errors, redis::EmailTask, requests::{SendCodePurposes, SendCodeRequest}, responses::CodeSentResponse}, utils::code_generator::{generate_code_token, generate_email_code}};

#[actix_web::post("/send_code")]
pub async fn send_code_endpoint(
    request: HttpRequest,
    body: Result<web::Json<SendCodeRequest>, Error>,
    redis: web::Data<Arc<Client>>
) -> Result<web::Json<CodeSentResponse>, Errors> {
    match body {
        Ok(body) => {
            // Check if email address is correct
            if body.validate().is_err() {
                return Err(Errors::BadRequest { what_invalid: "email field value" });
            }

            // Rate limits redis keys
            let addr_limit_key = format!("code_rate_limit:email:{}", body.email);
            let users_ip = match request.connection_info().realip_remote_addr() {
                Some(ip) => ip.to_string(),
                None => {
                    return Err(Errors::InternalServer { what: "IP address" });
                },
            };
            let ip_limit_key = format!("code_rate_limit:ip:{}", users_ip);

            // Check rate limit by ip address
            {
                let result: i64 = redis
                    .get(&ip_limit_key)
                    .await
                    .unwrap_or(0);

                if result > 5 {
                    let exp_time: i64 = redis.pttl(&ip_limit_key)
                        .await
                        .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                    let timestamp = (Utc::now() + Duration::milliseconds(exp_time)).timestamp();

                    return Err(Errors::IPRateLimit { 
                        how_much: (exp_time as u32) / 1000, 
                        timestamp: timestamp as u32
                    })
                }
            }

            // Check and update rate limit by email address (1 code in 3 minute)
            {
                let rate_result: Option<String> = redis
                    .set(&addr_limit_key, "1", Some(Expiration::EX(3 * 60)), Some(SetOptions::NX), false)
                    .await
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                match rate_result {
                    Some(_) => {},
                    None => {
                        let exp_time: i64 = redis.pttl(&addr_limit_key)
                            .await
                            .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                        let timestamp = (Utc::now() + Duration::milliseconds(exp_time)).timestamp();

                        return Err(Errors::EmailRateLimit { 
                            how_much: (exp_time as u32) / 1000, 
                            timestamp: timestamp as u32 
                        });
                    }
                };
            }

            // Update rate limit by ip address (5 codes in 10 minutes)
            {
                let _: i64 = redis
                    .incr(&ip_limit_key)
                    .await
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                let _: u8 = redis.expire(&ip_limit_key, 10 * 60, None)
                    .await
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;
            }

            // Generate code and token
            let email_code = generate_email_code();
            let email_token = generate_code_token();

            // Saving email code and token into storage for a day
            let key = format!("confirmcode:{}", body.email);
            let value = format!("{}:{}:{}", email_token, email_code, body.purpose.to_string());

            let expire_time = Utc::now() + Duration::seconds(24 * 60 * 60);

            let _: Option<String> = redis
                .set(&key, &value, Some(Expiration::EXAT(expire_time.timestamp())), None, false)
                .await
                .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

            // Add email task into queue to be processed by a SMTP service
            let mut replacements = HashMap::new();
            replacements.insert("CERTCODE".to_string(), email_code);

            redis.lpush::<i64, _, _>(configs::get_redis_mail_task_queue(), serde_json::to_string(&EmailTask {
                email: body.email.clone(),
                purpose: body.purpose.to_string(),
                replacements: replacements,
            }).unwrap())
                .await
                .map_err(|_| Errors::InternalServer { what: "broker" })?;

            // Success
            Ok(web::Json(
                CodeSentResponse::new(body.email.to_string(), email_token, expire_time.timestamp() as u64)
            ))
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
