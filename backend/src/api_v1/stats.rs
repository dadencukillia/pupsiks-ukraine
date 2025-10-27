use actix_web::{http::StatusCode, web, HttpResponse, Responder, Scope};
use crate::types::errors::Errors;

async fn not_found() -> Result<(), Errors> {
    Err(Errors::PageNotFound { 
        endpoints: Some(&[
            ("GET", "/api/v1/stats/users_count"),
        ])
    })
}

#[actix_web::get("/users_count")]
pub async fn users_count_endpoint() -> impl Responder {
    "0"
}

pub fn stats_scope() -> Scope {
    let stats_scope = web::scope("/stats")
        .service(users_count_endpoint)
        .default_service(web::route().to(not_found));

    stats_scope
}
