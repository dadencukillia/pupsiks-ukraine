use std::sync::Arc;
use chrono::{DateTime, Duration, Utc};
use fred::{
    prelude::*, 
    types::{
        Expiration, 
        Value
    }
};
use crate::utils::log_error::ResultLogger;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisRepoError {
    #[error("Some operations wasn't queued")]
    DidNotQueued
}

pub struct RedisRepo {
    redis: Arc<Client>
}

impl RedisRepo {
    pub fn new(redis_client: Arc<Client>) -> Self {
        Self {
            redis: redis_client
        }
    }

    /// Increases the value of the counter by 1
    pub async fn increase_by_one(&self, key: String, expire: Duration) -> Result<u64> {
        let pipeline = self.redis.multi();

        let result: Value = pipeline
            .incr(&key)
            .await
            .log_with_place_on_error("increase_by_one")?;

        if !result.is_queued() {
            return Err(RedisRepoError::DidNotQueued.into());
        }

        let result: Value = pipeline
            .expire(&key, expire.num_seconds(), None)
            .await
            .log_with_place_on_error("increase_by_one")?;

        if !result.is_queued() {
            return Err(RedisRepoError::DidNotQueued.into());
        }

        let (new_value, _): (u64, u8) = pipeline.exec(true)
            .await
            .log_with_place_on_error("increase_by_one")?;

        Ok(new_value)
    }

    /// Increases the value of the counter by specified value
    /// Set the negative value to decrease the counter value
    pub async fn increase_by(&self, key: String, value: i64, expire: Duration) -> Result<u64> {
        let pipeline = self.redis.multi();

        let result: Value = pipeline
            .incr_by(&key, value)
            .await
            .log_with_place_on_error("increase_by")?;

        if !result.is_queued() {
            return Err(RedisRepoError::DidNotQueued.into());
        }

        let result: Value = pipeline
            .expire(&key, expire.num_seconds(), None)
            .await
            .log_with_place_on_error("increase_by")?;

        if !result.is_queued() {
            return Err(RedisRepoError::DidNotQueued.into());
        }

        let (new_value, _): (u64, u8) = pipeline.exec(true)
            .await
            .log_with_place_on_error("increase_by")?;

        Ok(new_value)
    }

    /// Returns the value of stored in the Redis storage variable
    pub async fn get_value<T: FromValue>(&self, key: String) -> Result<Option<T>> {
        Ok(
            self.redis.get::<Option<T>, String>(key)
                .await
                .log_with_place_on_error("get_value")?
        )
    }

    /// Changes the value of the variable or create a variable with specified value
    /// replace_expire = true will change the TTL if the variable already exists
    /// replace_expire = false won't change the TTL if the variable already exists
    pub async fn set_value<T>(&self, key: String, value: T, expire: Duration, replace_expire: bool) -> Result<Option<()>> 
    where 
        T: TryInto<Value> + Send,
        T::Error: Into<Error> + Send
    {
        let a: Option<()> = self.redis.set(
            key, 
            value,
            Some(Expiration::EX(expire.num_seconds())), 
            if replace_expire { 
                None 
            } else { 
                Some(SetOptions::NX) 
            }, 
            false
        )
            .await
            .log_with_place_on_error("set_value")?;

        Ok(a)
    }

    /// Changes the value of the variable or create a variable with specified value
    /// Returns the previous value of the variable if it already exists
    /// replace_expire = true will change the TTL if the variable already exists
    /// replace_expire = false won't change the TTL if the variable already exists
    pub async fn set_value_return_previous<T>(&self, key: String, value: T, expire: Duration, replace_expire: bool) -> Result<Option<T>> 
    where 
        T: TryInto<Value> + FromValue + Send,
        T::Error: Into<Error> + Send
    {
        let a: Option<T> = self.redis.set(
            key, 
            value,
            Some(Expiration::EX(expire.num_seconds())), 
            if replace_expire { 
                None 
            } else { 
                Some(SetOptions::NX) 
            }, 
            false
        )
            .await
            .log_with_place_on_error("set_value_return_previous")?;

        Ok(a)
    }

    /// Removes the variable by the key from the Redis storage
    pub async fn delete_by_key(&self, key: String) -> Result<u64> {
        let count: u64 = self.redis
            .del(&key)
            .await
            .log_with_place_on_error("delete_by_key")?;

        Ok(count)
    }

    /// Returns the time to left and UTC expiration time of the specified variable
    pub async fn get_ttl(&self, key: String) -> Result<(Duration, DateTime<Utc>)> {
        let timestamp_milli: i64 = self.redis
            .pexpire_time(key)
            .await
            .log_with_place_on_error("get_ttl")?;

        let now = Utc::now();
        let timestamp = DateTime::from_timestamp_millis(timestamp_milli).unwrap_or(now);
        Ok((timestamp - now, timestamp))
    }

    /// Adds the object in a queue by the specified key
    pub async fn lpush<T>(&self, key: String, value: T) -> Result<u32>
    where 
        T: TryInto<Value> + Send,
        T::Error: Into<Error> + Send
    {
        let new_size: u32 = self.redis.lpush(key, value)
            .await
            .log_with_place_on_error("lpush")?;

        Ok(new_size)
    }
}
