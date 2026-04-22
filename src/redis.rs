use std::env;

use axum::http::StatusCode;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

use crate::{RedisCon, errors::PersErrors};

pub type RedisPool = Pool<RedisConnectionManager>;

/**
 * this will create a async pool of reds connnection
 * it is been added to app_state
 * where ever we need redis for cashing , we will get 1 con from asyn redis pool and connection with redis for fast set/get/delete value on servers ram memory
 */
pub async fn create_redis_pool() -> Result<RedisPool, PersErrors> {
    let redis_con_url = env::var("REDIS_URL").map_err(|e| {
        PersErrors::new(
            format!("getting error in redis con url - {}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;
    let manager = RedisConnectionManager::new(redis_con_url).map_err(|e| {
        PersErrors::new(
            format!("getting error while connecting to redis {}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let r_pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .await
        .map_err(|e| {
            PersErrors::new(
                format!(
                    "getting error in redis while creating connection pool - {}",
                    e
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    return Ok(r_pool);
}



pub async fn get_redis_conn_from_pool(r_pool: &RedisPool) -> Result<RedisCon<'_>, PersErrors> {
    let redis_con = r_pool.get().await.map_err(|e| {
        PersErrors::new(
            format!(
                "error while getting connnection for redis connection pool => {}",
                e
            ),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(redis_con)
}
