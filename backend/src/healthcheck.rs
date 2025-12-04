use std::sync::Arc;

use actix_web::{HttpResponse, HttpResponseBuilder, Resource, http::StatusCode, web};
use fred::prelude::*;
use sea_orm::DatabaseConnection;

use crate::utils::log_error::ResultLogger;

/// Checks the database and redis connections
/// Returns simple text messages
/// Useful for Docker healthcheck
pub async fn healthcheck_endpoint(
    db: web::Data<Arc<DatabaseConnection>>,
    redis: web::Data<Arc<Client>>
) -> HttpResponse {
    if let Err(_) = db.ping().await.log_with_place_on_error("healthcheck_endpoint") {
        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Database connection is unhealthy")
    } else if let Err(_) = redis.ping::<String>(None).await.log_with_place_on_error("healthcheck_endpoint") {
        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Redis connection is unhealthy")
    } else {
        HttpResponseBuilder::new(StatusCode::OK)
            .body("Everything is healthy!")
    }
}

/// Adds the /healthcheck endpoint that checks the database and redis connections
pub fn healthcheck_resource(
    db: Arc<DatabaseConnection>,
    redis: Arc<Client>
) -> Resource {
    let healthcheck_resource = web::resource("/healthcheck")
        .app_data(web::Data::new(db))
        .app_data(web::Data::new(redis))
        .route(web::get().to(healthcheck_endpoint));

    healthcheck_resource
}
