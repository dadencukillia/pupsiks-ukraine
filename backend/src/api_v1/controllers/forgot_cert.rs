use actix_web::{web, Error, HttpRequest};
use chrono::Duration;
use short_uuid::ShortUuid;
use crate::{
    api_v1::{
        repos::{
            CertRepo, 
            RedisRepo
        }, 
        services::{
            email::send_forgot_cert, 
            rate_limits
        }, 
        types::{
            errors::Errors, 
            requests::ForgotCertRequest, 
            responses::success::CertEmailResponse
        }
    }, 
    utils::log_error::ResultLogger
};

#[actix_web::post("/cert/forgot")]
pub async fn forgot_cert_endpoint(
    request: HttpRequest,
    body: Result<web::Json<ForgotCertRequest>, Error>,
    redis: web::Data<RedisRepo>,
    cert_repo: web::Data<CertRepo>
) -> Result<web::Json<CertEmailResponse>, Errors> {
    let place_name = "POST /api/v1/cert/forgot";

    match body.log_with_place_on_error(place_name){
        Ok(unclear_body) => {
            let body = unclear_body.trim();

            let user_ip = match request.connection_info().realip_remote_addr() {
                Some(ip) => ip.to_string(),
                None => {
                    return Err(Errors::InternalServer { what: "IP address" });
                },
            };

            // Check rate limit by IP address
            if !rate_limits::check_rate_counter(
                redis.as_ref(), 
                "forgot", &user_ip,
                3
            ).await {
                let ttl = rate_limits::get_rate_time(
                    redis.as_ref(),
                    "forgot", &user_ip
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
                "forgot", &body.email,
                1
            ).await {
                let ttl = rate_limits::get_rate_time(
                    redis.as_ref(),
                    "forgot", &body.email
                )
                    .await
                    .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                return Err(Errors::EmailRateLimit { 
                    how_much: ttl.0.num_seconds() as u32,
                    timestamp: ttl.1.timestamp() as u64
                })
            }

            // Find cert
            let find_option = cert_repo.find_cert_by_email(body.email.clone())
                .await
                .map_err(|_| Errors::InternalServer { what: "DB" })?;

            if let Some(certificate) = find_option {
                // Update rate limit by IP address
                let _ = rate_limits::increate_rate_counter(
                    redis.as_ref(), 
                    "forgot", &user_ip, 
                    Duration::minutes(10)
                ).await;

                // Update rate limit by email address
                let _ = rate_limits::increate_rate_counter(
                    redis.as_ref(), 
                    "forgot", &body.email, 
                    Duration::days(1)
                ).await;

                // Send an email letter
                send_forgot_cert(
                    &redis, 
                    &body.email, 
                    &ShortUuid::from_uuid(&certificate.id).to_string()
                )
                    .await
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
