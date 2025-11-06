use std::sync::Arc;

use actix_web::{http::StatusCode, web, HttpResponse, Responder, Scope};
use fred::prelude::{Client, FredResult, KeysInterface};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};
use crate::{models::cert, types::{errors::Errors, responses::success::StatsUserCountResponse}, utils::log_error::ResultLogger};

async fn not_found() -> Result<(), Errors> {
    Err(Errors::PageNotFound { 
        endpoints: Some(&[
            ("GET", "/api/v1/stats/users_count"),
        ])
    })
}

#[actix_web::get("/users_count")]
pub async fn users_count_endpoint(
    db: web::Data<DatabaseConnection>,
    redis: web::Data<Arc<Client>>
) -> Result<web::Json<StatsUserCountResponse>, Errors> {
    let place_name = "GET /api/v1/stats/users_count";

    if let Ok(Some(cache_data)) = redis
        .get::<Option<u64>, _>("stats:users_count")
        .await
        .log_with_place_on_error(place_name)
    {
        return Ok(web::Json(
            StatsUserCountResponse::new(cache_data)
        ));
    }

    let users_count = cert::Entity::find()
        .count(db.as_ref())
        .await
        .log_with_place_on_error(place_name)
        .map_err(|_| Errors::InternalServer { what: "DB" })?;

    let _: FredResult<Option<u64>> = redis
        .set("stats:users_count", users_count, Some(fred::types::Expiration::EX(24 * 60 * 60)), None, false)
        .await
        .log_with_place_on_error(place_name);

    Ok(web::Json(
        StatsUserCountResponse::new(users_count)
    ))
}

pub fn stats_scope() -> Scope {
    let stats_scope = web::scope("/stats")
        .service(users_count_endpoint)
        .default_service(web::route().to(not_found));

    stats_scope
}
