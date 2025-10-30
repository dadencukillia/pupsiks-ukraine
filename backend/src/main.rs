use std::{sync::Arc, time::Duration};

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use fred::prelude::*;
use sea_orm::{Database, DatabaseConnection};
use crate::models::cert;

mod configs;
mod api_v1;
mod models;
mod types;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        Env::default().default_filter_or("info")
    );

    let db: DatabaseConnection = Database::connect(configs::get_db_url()).await.unwrap();
    db.get_schema_builder()
        .register(cert::Entity)
        .sync(&db)
        .await.unwrap();

    let redis = Builder::from_config(configs::get_redis_config())
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(5);
            config.tcp = TcpConfig {
                nodelay: Some(true),
                ..Default::default()
            };
        })
        .set_policy(ReconnectPolicy::new_constant(0, 5))
        .build().unwrap();
    redis.init().await.unwrap();

    let redis_clone = Arc::new(redis);

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(redis_clone.clone()))
            .service(api_v1::api_v1_scope())

    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
