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

    pub async fn get_value<T: FromValue>(&self, key: String) -> Result<Option<T>> {
        Ok(
            self.redis.get::<Option<T>, String>(key)
                .await
                .log_with_place_on_error("get_value")?
        )
    }

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

    pub async fn delete_by_key(&self, key: String) -> Result<u64> {
        let count: u64 = self.redis
            .del(&key)
            .await
            .log_with_place_on_error("delete_by_key")?;

        Ok(count)
    }

    pub async fn get_ttl(&self, key: String) -> Result<(Duration, DateTime<Utc>)> {
        let timestamp_milli: i64 = self.redis
            .pexpire_time(key)
            .await
            .log_with_place_on_error("get_ttl")?;

        let now = Utc::now();
        let timestamp = DateTime::from_timestamp_millis(timestamp_milli).unwrap_or(now);
        Ok((timestamp - now, timestamp))
    }

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
