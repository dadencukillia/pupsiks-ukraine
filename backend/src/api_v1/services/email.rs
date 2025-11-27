use std::collections::HashMap;
use anyhow::Result;
use crate::api_v1::{
    repos::RedisRepo, 
    types::redis::EmailTask
};

pub async fn send_create_code(
    redis: &RedisRepo,
    email: &str, code: &str
) -> Result<()> {
    let mut replacements = HashMap::new();
    replacements.insert("CERTCODE".to_string(), code.to_string());

    redis.lpush("email_jobs".to_string(), serde_json::to_string(&EmailTask {
        email: email.to_string(),
        purpose: "create".to_string(),
        replacements: replacements,
    }).unwrap()).await?;

    Ok(())
}

pub async fn send_delete_code(
    redis: &RedisRepo,
    email: &str, code: &str
) -> Result<()> {
    let mut replacements = HashMap::new();
    replacements.insert("CERTCODE".to_string(), code.to_string());

    redis.lpush("email_jobs".to_string(), serde_json::to_string(&EmailTask {
        email: email.to_string(),
        purpose: "delete".to_string(),
        replacements: replacements,
    }).unwrap()).await?;

    Ok(())
}

pub async fn send_forgot_cert(
    redis: &RedisRepo,
    email: &str, cert_id: &str
) -> Result<()> {
    let mut replacements = HashMap::new();
    replacements.insert("CERTID".to_string(), cert_id.to_string());

    redis.lpush("email_jobs".to_string(), serde_json::to_string(&EmailTask {
        email: email.to_string(),
        purpose: "forgot".to_string(),
        replacements: replacements,
    }).unwrap()).await?;

    Ok(())
}
