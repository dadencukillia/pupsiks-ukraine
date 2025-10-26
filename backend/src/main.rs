use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};
use crate::models::cert;

mod configs;
mod api_v1;
mod models;
mod types;

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

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(web::Data::new(db.clone()))
            .service(api_v1::api_v1_scope())

    }).bind(("0.0.0.0", 8080))?
        .run()
        .await
}
