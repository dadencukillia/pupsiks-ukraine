use std::{collections::HashMap, sync::Arc};

use actix_web::{web, Error, Responder};
use chrono::{Duration, Utc};
use fred::{prelude::{Client, HashesInterface, KeysInterface, ListInterface}, types::{Expiration, SetOptions}};
use validator::Validate;

use crate::{configs, types::{errors::Errors, redis_email_task::EmailTask, requests::{SendCodePurposes, SendCodeRequest}}};

#[actix_web::post("/send_code")]
pub async fn send_code_endpoint(
    body: Result<web::Json<SendCodeRequest>, Error>,
    redis: web::Data<Arc<Client>>
) -> Result<&'static str, Errors> {
    match body {
        Ok(request) => {
            if request.validate().is_err() {
                return Err(Errors::BadRequest { what_invalid: "email field value" });
            }

            let key = format!("rate_limit:email:{}", request.email);
            let rate_result: Option<String> = redis
                .set(&key, "1", Some(Expiration::EX(3 * 60)), Some(SetOptions::NX), false)
                .await
                .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

            match rate_result {
                Some(_) => {},
                None => {
                    let exp_time: i64 = redis.pttl(&key)
                        .await
                        .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                    let timestamp = (Utc::now() + Duration::milliseconds(exp_time)).timestamp();

                    return Err(Errors::EmailRateLimit { 
                        how_much: (exp_time as u32) / 1000, 
                        timestamp: timestamp as u32 
                    });
                }
            };

            redis.lpush::<i64, _, _>(configs::get_redis_mail_task_queue(), serde_json::to_string(&EmailTask {
                email: request.email.clone(),
                purpose: request.purpose.to_string(),
                replacements: HashMap::<String, String>::new(),
            }).unwrap())
                .await
                .map_err(|_| Errors::InternalServer { what: "broker" })?;

            Ok("Hey")
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
