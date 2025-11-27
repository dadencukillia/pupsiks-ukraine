use std::{future::Ready, sync::Arc, time::Duration};
use actix_extensible_rate_limit::{RateLimiter, backend::{SimpleInput, SimpleInputFunctionBuilder, SimpleOutput, memory::InMemoryBackend}};
use actix_web::{Error, ResponseError, Result, Scope, dev::{ServiceFactory, ServiceRequest}, web::{self, Data}};
use fred::prelude::Client;
use sea_orm::DatabaseConnection;
use crate::api_v1::{repos::{CertRepo, RedisRepo}, types::errors::Errors};

mod code_confirmation;
mod create_cert;
mod delete_cert;
mod forgot_cert;
mod get_cert;
mod stats;

const BODY_PAYLOAD_LIMIT: usize = 4096; // 4 Kb

async fn not_found() -> Result<(), Errors> {
    Err(Errors::PageNotFound { 
        endpoints: Some(&[
            ("POST", "/api/v1/cert/forgot"),
            ("GET", "/api/v1/cert/{uuid}"),
            ("POST", "/api/v1/cert"),
            ("DELETE", "/api/v1/cert"),
            ("POST", "/api/v1/send_code"),
            ("ANY", "/api/v1/stats")
        ])
    })
}

fn rate_limit_middleware() -> RateLimiter<InMemoryBackend, SimpleOutput, impl Fn(&ServiceRequest) -> Ready<Result<SimpleInput, actix_web::Error>> + 'static>  {
    let rate_limit_backend = InMemoryBackend::builder().build();
    let rate_limit_input = SimpleInputFunctionBuilder::new(Duration::from_secs(1), 3)
        .real_ip_key()
        .build();
    let rate_limit_middleware = RateLimiter::builder(rate_limit_backend.clone(), rate_limit_input)
        .add_headers()
        .request_denied_response(|_| {
            Errors::RequestsRateLimit.error_response()
        })
        .build();

    rate_limit_middleware
}

fn payload_limit() -> web::PayloadConfig {
    web::PayloadConfig::default()
        .limit(BODY_PAYLOAD_LIMIT)
}

fn json_payload_limit() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(BODY_PAYLOAD_LIMIT)
        .error_handler(|_, _| -> Error {
            Errors::PayloadTooLarge { bytes_limit: BODY_PAYLOAD_LIMIT }.into()
        })
}

pub fn api_v1_scope(
    database_connection: Arc<DatabaseConnection>,
    redis_client: Arc<Client>
) -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse<actix_web::body::EitherBody<actix_web::body::BoxBody>>, Error = actix_web::Error, InitError = ()>> {
    let api_scope = web::scope("/api/v1")
        .wrap(rate_limit_middleware())
        .app_data(payload_limit())
        .app_data(json_payload_limit())
        .app_data(Data::new(CertRepo::new(database_connection)))
        .app_data(Data::new(RedisRepo::new(redis_client)))
        .service(get_cert::get_cert_endpoint)
        .service(create_cert::create_cert_endpoint)
        .service(delete_cert::delete_cert_endpoint)
        .service(forgot_cert::forgot_cert_endpoint)
        .service(code_confirmation::send_code_endpoint)
        .service(stats::stats_scope())
        .default_service(web::route().to(not_found));

    api_scope
}
