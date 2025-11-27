use actix_web::{Error, web};
use chrono::{Duration, Utc};
use validator::Validate;
use crate::{
    api_v1::{
        repos::{
            CertRepo, 
            RedisRepo
        }, 
        services::{
            cache, 
            codes::{
                self, 
                VerificationResult
            }, 
            rate_limits
        }, 
        types::{
            errors::Errors, 
            requests::DeleteCertRequest, 
            responses::success::CertIdResponse
        }
    }, 
    utils::log_error::ResultLogger
};

#[actix_web::delete("/cert")]
pub async fn delete_cert_endpoint(
    body: Result<web::Json<DeleteCertRequest>, Error>,
    redis: web::Data<RedisRepo>,
    cert_repo: web::Data<CertRepo>
) -> Result<web::Json<CertIdResponse>, Errors> {
    let place_name = "DELETE /api/v1/cert";

    match body.log_with_place_on_error(place_name) {
        Ok(body_unclear) => {
            // Cleaning and validating body
            let body = body_unclear.trim();

            if body
                .validate()
                .log_with_place_on_error(place_name)
                .is_err() {
                return Err(Errors::BadRequest { what_invalid: "field values" });
            }

            // Veryfing code
            let verification_result = codes::verify_email_code(
                redis.as_ref(), 
                &body.email, 
                &body.token, 
                &body.code
            ).await;

            match verification_result {
                VerificationResult::Ok { purpose } => {
                    if purpose != "delete" {
                        // When created code had another purpose
                        return Err(Errors::InvalidRoute { correct_route: "POST /api/v1/cert" });
                    }

                    // Delete code from Redis
                    codes::remove_code_from_storage(redis.as_ref(), &body.email)
                        .await
                        .map_err(|_| Errors::InternalServer { what: "cache storage" })?;

                    // Reset rate counter by email
                    let _ = rate_limits::reset_rate_counter(
                        redis.as_ref(), 
                        "code", &body.email
                    ).await;

                    // Get cert to remove
                    let cert_option = cert_repo.find_cert_by_email(body.email.clone())
                        .await
                        .map_err(|_| Errors::InternalServer { what: "DB" })?;

                    if cert_option.is_none() {
                        return Err(Errors::ResourceNotFound { what: "certificate" });
                    }

                    let cert = cert_option.unwrap();

                    // Execute delete operation
                    let deletion_count = cert_repo.remove_cert_by_id_and_email(cert.id.clone(), body.email.clone())
                        .await
                        .map_err(|_| Errors::InternalServer { what: "DB" })?;

                    if deletion_count == 0 {
                        Err(Errors::ResourceNotFound { what: "certificate" })
                    } else {
                        let _ = redis.increase_by(
                            cache::get_key("stats:users_count"), 
                            -1, 
                            Duration::days(1)
                        ).await;

                        Ok(web::Json(
                            CertIdResponse::new(&cert.id)
                        ))
                    }
                },
                VerificationResult::InvalidToken => Err(Errors::InvalidToken),
                VerificationResult::NotFound => Err(Errors::ResourceNotFound { what: "code record" }),
                VerificationResult::UnknownError( .. ) => Err(Errors::InternalServer { what: "code verification" }),
                VerificationResult::InvalidCode => {
                    if rate_limits::check_rate_counter(
                        redis.as_ref(), 
                        "token_tries", &body.token, 
                        5
                    ).await {
                        rate_limits::increate_rate_counter(
                            redis.as_ref(), 
                            "token_tries", &body.token, 
                            Duration::days(1)
                        )
                            .await
                            .map_err(|_| Errors::InvalidCode)?;

                        Err(Errors::InvalidCode)
                    } else {
                        let block_duration = Duration::minutes(15);
                        let block_timestamp = Utc::now() + block_duration;

                        let _ = codes::remove_code_from_storage(
                            &redis, &body.email
                        ).await;

                        let _ = redis.set_value(
                            rate_limits::get_key("code", &body.email), 
                            10000, 
                            block_duration.clone(), true
                        ).await;

                        let _ = rate_limits::reset_rate_counter(
                            &redis,
                            "token_tries", &body.token
                        ).await;

                        Err(Errors::TriesOut { 
                            how_much: block_duration.num_seconds() as u32, 
                            timestamp: block_timestamp.timestamp() as u64
                        })
                    }
                },
            }
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
