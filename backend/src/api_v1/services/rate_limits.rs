use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use crate::api_v1::repos::RedisRepo;

pub fn get_key(
    realm: &str, key: &str
) -> String {
    format!("rate_limit:{}:{}", realm, key)
}

pub async fn get_rate_counter(
    redis: &RedisRepo,
    realm: &str, key: &str
) -> u64 {
    redis.get_value(get_key(realm, key)).await
        .unwrap_or(None)
        .unwrap_or(0)
}

pub async fn increate_rate_counter(
    redis: &RedisRepo,
    realm: &str, key: &str, exp: Duration
) -> Result<u64> {
    redis.increase_by_one(get_key(realm, key), exp).await
}

pub async fn check_rate_counter(
    redis: &RedisRepo,
    realm: &str, key: &str, limit: u64
) -> bool {
    get_rate_counter(redis, realm, key).await < limit
}

pub async fn get_rate_time(
    redis: &RedisRepo,
    realm: &str, key: &str
) -> Result<(Duration, DateTime<Utc>)> {
    redis.get_ttl(get_key(realm, key)).await
}

pub async fn reset_rate_counter(
    redis: &RedisRepo,
    realm: &str, key: &str
) -> Result<()> {
    redis.delete_by_key(get_key(realm, key)).await?;

    Ok(())
}
