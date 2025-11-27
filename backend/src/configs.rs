use std::env;
use fred::prelude::Config;

pub fn get_db_url() -> String {
    let user = env::var("DB_USER").unwrap();
    let pass = env::var("DB_PASS").unwrap();

    format!("postgres://{}:{}@db:5432/pupsiks", user, pass)
}

pub fn get_redis_config() -> Config {
    Config::from_url("redis://redis:6379").unwrap()
}
