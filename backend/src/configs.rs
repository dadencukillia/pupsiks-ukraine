use std::env;

use fred::prelude::Config;

pub fn get_db_url() -> String {
    let user = env::var("DB_USER").unwrap_or("crocoby".to_string());
    let pass = env::var("DB_PASS").unwrap_or("Z3qNy5sJqrft2L36".to_string());

    format!("postgres://{}:{}@127.0.0.1:5432/pupsiks", user, pass)
}

pub fn get_redis_config() -> Config {
    Config::from_url("redis://localhost:6379").unwrap()
}

pub fn get_redis_mail_task_queue() -> String {
    env::var("EMAIL_QUEUE").unwrap_or("email_jobs".to_string())
}
