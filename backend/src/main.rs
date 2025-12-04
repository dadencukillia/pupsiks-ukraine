use std::sync::Arc;
use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;

mod configs;
mod connections;
mod api_v1;
mod utils;
mod healthcheck;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set-up logger
    env_logger::init_from_env(
        Env::default().default_filter_or("info")
    );

    // Set-up PostgreSQL and Redis connection
    let db = connections::get_database_connection().await.unwrap();
    let redis = connections::get_redis_client().await.unwrap();

    let db_arc = Arc::new(db);
    let redis_arc = Arc::new(redis);

    // Create and configurate Actix web server
    HttpServer::new(move || {
        let logger_middleware = Logger::default();

        App::new()
            .wrap(logger_middleware)
            .service(healthcheck::healthcheck_resource(db_arc.clone(), redis_arc.clone()))
            .service(api_v1::api_v1_scope(db_arc.clone(), redis_arc.clone()))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
