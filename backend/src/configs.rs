use std::env;

pub fn get_db_url() -> String {
    let user = env::var("DB_USER").unwrap_or("crocoby".to_string());
    let pass = env::var("DB_PASS").unwrap_or("Z3qNy5sJqrft2L36".to_string());

    format!("postgres://{}:{}@127.0.0.1:5432/pupsiks", user, pass)
}
