use actix_web::{web, Error, HttpRequest};
use chrono::Duration;
use validator::Validate;
use crate::{
    api_v1::{
        repos::{
            CertRepo, 
            RedisRepo
        }, 
        services::{
            codes, 
            email, 
            rate_limits
        }, 
        types::{
            errors::Errors, 
            requests::{
                SendCodePurposes, 
                SendCodeRequest
            }, 
            responses::success::CodeSentResponse
        }
    }, 
    utils::{
        log_error::ResultLogger, 
        uuid::get_uuid
    }
};

#[actix_web::post("/send_code")]
pub async fn send_code_endpoint(
    request: HttpRequest,
    body: Result<web::Json<SendCodeRequest>, Error>,
    redis: web::Data<RedisRepo>,
    cert_repo: web::Data<CertRepo>
) -> Result<web::Json<CodeSentResponse>, Errors> {
    let place_name = "POST /api/v1/send_code";

    match body.log_with_place_on_error(place_name) {
        Ok(body_unclear) => {
            let body = body_unclear.trim();

            // Check if email address is correct
            if body.validate()
                .log_with_place_on_error(place_name)
                .is_err() {
                return Err(Errors::BadRequest { what_invalid: "email field value" });
            }

            let user_ip = match request.connection_info().realip_remote_addr() {
                Some(ip) => ip.to_string(),
                None => {
                    return Err(Errors::InternalServer { what: "IP address" });
                },
            };

            // Check rate limit by IP address
            if !rate_limits::check_rate_counter(
                redis.as_ref(), 
                "code", &user_ip,
                5
            ).await {
                let ttl = rate_limits::get_rate_time(
                    redis.as_ref(),
                    "code", &user_ip
                )
                    .await
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                return Err(Errors::IPRateLimit { 
                    how_much: ttl.0.num_seconds() as u32,
                    timestamp: ttl.1.timestamp() as u64
                })
            }

            // Check rate limit by email address
            if !rate_limits::check_rate_counter(
                redis.as_ref(), 
                "code", &body.email,
                1
            ).await {
                let ttl = rate_limits::get_rate_time(
                    redis.as_ref(),
                    "code", &body.email
                )
                    .await
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                return Err(Errors::EmailRateLimit { 
                    how_much: ttl.0.num_seconds() as u32,
                    timestamp: ttl.1.timestamp() as u64
                })
            }

            // Check special cases
            match body.purpose {
                SendCodePurposes::ConfirmCreation => {
                    let cert_to_check = cert_repo.find_cert_by_email(body.email.clone())
                        .await
                        .map_err(|_| Errors::InternalServer { what: "DB" })?;

                    if cert_to_check.is_some() {
                        return Err(Errors::AlreadyExists { what: "certificate with this email" });
                    }
                },
                SendCodePurposes::ConfirmDeletion { ref id } => {
                    if let Some(uuid) = get_uuid(id) {
                        let cert_to_check = cert_repo.find_cert_by_id(uuid)
                            .await
                            .map_err(|_| Errors::InternalServer { what: "DB" })?;

                        if let Some(cert) = cert_to_check {
                            if cert.email != body.email {
                                return Err(Errors::InvalidEmail);
                            }
                        } else {
                            return Err(Errors::ResourceNotFound { what: "certificate with this ID" });
                        }
                    } else {
                        return Err(Errors::BadRequest { what_invalid: "id field value" });
                    };
                }
            };

            // Update rate limit by IP address
            let _ = rate_limits::increate_rate_counter(
                redis.as_ref(), 
                "code", &user_ip, 
                Duration::minutes(10)
            ).await;

            // Update rate limit by email address
            let _ = rate_limits::increate_rate_counter(
                redis.as_ref(), 
                "code", &body.email, 
                Duration::minutes(3)
            ).await;

            // Generating code and token
            let email_code = codes::generate_email_code();
            let email_token = codes::generate_code_token();

            // Saving email code and token into storage for a day
            let expire_time = codes::save_code_in_storage(
                redis.as_ref(), 
                &body.email, 
                &body.purpose.to_string(), 
                &email_code, &email_token
            )
                .await
                .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

            // Adding email task into queue to be processed by a SMTP service
            match body.purpose {
                SendCodePurposes::ConfirmCreation => {
                    email::send_create_code(
                        redis.as_ref(), &body.email, &email_code
                    )
                        .await
                        .map_err(|_| Errors::InternalServer { what: "broker" })?;
                },
                SendCodePurposes::ConfirmDeletion { .. } => {
                    email::send_delete_code(
                        redis.as_ref(), &body.email, &email_code
                    )
                        .await
                        .map_err(|_| Errors::InternalServer { what: "broker" })?;
                }
            }

            // Success
            Ok(web::Json(
                CodeSentResponse::new(body.email.to_string(), email_token, expire_time.timestamp() as u64)
            ))
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
