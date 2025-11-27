use sea_orm::schema::SchemaBuilder;

mod controllers;
mod models;
mod types;
mod repos;
mod services;

pub use controllers::api_v1_scope;

pub fn register_cert_in_db_schema(schema_builder: SchemaBuilder) -> SchemaBuilder {
    schema_builder.register(models::cert::Entity)
}
