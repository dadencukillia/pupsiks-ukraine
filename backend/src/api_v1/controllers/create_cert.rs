use actix_web::{Error, web};
use chrono::{Duration, Utc};
use uuid::Uuid;
use validator::Validate;
use crate::{
    api_v1::{
        repos::{
            CertModel, 
            CertRepo, 
            CreationError, 
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
            requests::CreateCertRequest, 
            responses::success::CertificateResponse
        }
    }, 
    utils::log_error::ResultLogger
};

#[actix_web::post("/cert")]
pub async fn create_cert_endpoint(
    body: Result<web::Json<CreateCertRequest>, Error>,
    redis: web::Data<RedisRepo>,
    cert_repo: web::Data<CertRepo>
) -> Result<web::Json<CertificateResponse>, Errors> {
    let place_name = "POST /api/v1/cert";

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
                    if purpose != "create" {
                        // When created code had another purpose
                        return Err(Errors::InvalidRoute { correct_route: "DELETE /api/v1/cert" });
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

                    // Creating and saving certificate to DB
                    let cert_uuid = Uuid::new_v4();
                    let creation_result = cert_repo.create_cert(CertModel {
                        id: cert_uuid.clone(),
                        email: body.email,
                        name: body.name.clone(),
                        title: body.title.clone()
                    }).await;

                    match creation_result {
                        Ok(_) => {},
                        Err(CreationError::UniqueErr) => {
                            return Err(Errors::AlreadyExists { what: "certificate with this email" });
                        },
                        Err(CreationError::Another( .. )) => {
                            return Err(Errors::InternalServer { what: "DB" });
                        }
                    };

                    // Updating stats
                    let _ = redis.increase_by_one(
                        cache::get_key("stats:users_count"), 
                        Duration::days(1)
                    ).await;

                    // Returning certificate data
                    Ok(web::Json(CertificateResponse::new(
                        &cert_uuid, 
                        body.name.to_string(), 
                        body.title.to_string()
                    )))
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
