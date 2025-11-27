use std::time::Duration;
use fred::prelude::*;
use anyhow::Result;
use sea_orm::{Database, DatabaseConnection};
use crate::{
    api_v1::register_cert_in_db_schema, 
    configs
};

pub async fn get_redis_client() -> Result<Client> {
    let redis: Client = Builder::from_config(configs::get_redis_config())
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(5);
            config.tcp = TcpConfig {
                nodelay: Some(true),
                ..Default::default()
            };
        })
        .set_policy(ReconnectPolicy::new_constant(0, 5)).build()?;
    redis.init().await?;

    Ok(redis)
}

pub async fn get_database_connection() -> Result<DatabaseConnection> {
    let db: DatabaseConnection = Database::connect(configs::get_db_url()).await?;
    register_cert_in_db_schema(
        db.get_schema_builder()
    ).sync(&db).await?;

    Ok(db)
}
