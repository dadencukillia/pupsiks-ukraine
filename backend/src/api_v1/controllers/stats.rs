use chrono::Duration;
use actix_web::{web, Scope};
use crate::api_v1::{
    repos::{CertRepo, RedisRepo}, 
    services::cache, 
    types::{
        errors::Errors, 
        responses::success::StatsUserCountResponse
    }
};

async fn not_found() -> Result<(), Errors> {
    Err(Errors::PageNotFound { 
        endpoints: Some(&[
            ("GET", "/api/v1/stats/users_count"),
        ])
    })
}

#[actix_web::get("/users_count")]
pub async fn users_count_endpoint(
    cert_repo: web::Data<CertRepo>,
    redis: web::Data<RedisRepo>
) -> Result<web::Json<StatsUserCountResponse>, Errors> {
    // Receive a cached users count
    let cache_key = "stats:users_count";
    let cache_option = cache::get_cache(redis.as_ref(), cache_key.to_string()).await;

    if let Ok(Some(cached_data)) = cache_option {
        return Ok(web::Json(
            StatsUserCountResponse::new(cached_data)
        ))
    }

    // Receive the users count from the data base if not cached
    let count = cert_repo.count_all()
        .await
        .map_err(|_| Errors::InternalServer { what: "DB" })?;

    // Cache the received users count
    let _ = cache::set_cache(
        redis.as_ref(), 
        &cache_key, 
        count, 
        Duration::days(1)
    ).await;

    Ok(web::Json(
        StatsUserCountResponse::new(count)
    ))
}

pub fn stats_scope() -> Scope {
    let stats_scope = web::scope("/stats")
        .service(users_count_endpoint)
        .default_service(web::route().to(not_found));

    stats_scope
}
