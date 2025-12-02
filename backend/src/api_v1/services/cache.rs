use anyhow::Result;
use chrono::Duration;
use fred::{
    error::Error, 
    types::{
        FromValue, 
        Value
    }
};
use crate::api_v1::repos::RedisRepo;

/// Turns a cache key into the specialized Redis key
pub fn get_key(
    key: &str
) -> String {
    format!("cache:{}", key)
}

/// Returns the stored cache value by the cache key
pub async fn get_cache<T: FromValue>(
    redis: &RedisRepo,
    key: String
) -> Result<Option<T>> {
    redis.get_value(get_key(&key)).await
}

/// Caches a value by the cache key for a some time
pub async fn set_cache<T>(
    redis: &RedisRepo,
    key: &str, value: T, exp: Duration
) -> Result<()>
where 
    T: TryInto<Value> + Send,
    T::Error: Into<Error> + Send
{
    redis.set_value(get_key(key), value, exp, true).await?;

    Ok(())
}
