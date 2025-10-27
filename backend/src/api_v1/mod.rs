use actix_web::{http::StatusCode, web, HttpResponse, Result, Scope};

use crate::types::errors::Errors;

mod get_cert;
mod create_cert;
mod delete_cert;
mod code_confirmation;
mod stats;

async fn not_found() -> Result<(), Errors> {
    Err(Errors::PageNotFound { 
        endpoints: Some(&[
            ("GET", "/api/v1/cert/{uuid}"),
            ("POST", "/api/v1/cert"),
            ("DELETE", "/api/v1/cert"),
            ("POST", "/api/v1/send_code"),
            ("ANY", "/api/v1/stats")
        ])
    })
}

pub fn api_v1_scope() -> Scope {
    let api_scope = web::scope("/api/v1")
        .service(get_cert::get_cert_endpoint)
        .service(create_cert::create_cert_endpoint)
        .service(delete_cert::delete_cert_endpoint)
        .service(code_confirmation::send_code_endpoint)
        .service(stats::stats_scope())
        .default_service(web::route().to(not_found));

    api_scope
}
