use std::env;

use axum::http::StatusCode;

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;

use crate::{DbCon, DbPool, errors::PersErrors};

pub async fn create_db_pool() -> Result<DbPool, PersErrors> {

    // db postgreslq url
    let db_url = env::var("DATABASE_URL")
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);

    let pool = Pool::builder().build(config).await.map_err(|e| {
        PersErrors::new(
            format!("failed to create db pool: {}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(pool)
}
/**
 * we will call this function to get 1 conn from the pool
 * it is using connection spawn blocking , so this getting con from pool will block the thread , so it will be placed to worker thread for processing/waaiting
 * and this main thread will be empty for other task
 */
pub async fn get_connection_from_pool(pool: &DbPool) -> Result<DbCon<'_>, PersErrors> {
    // we are awaiting for this blocking task to first finish , then only we will move forward
    // this blocking task will run on seperate blocking threads
    let db_con = pool.get().await.map_err(|e| {
        PersErrors::new(
            format!("failed to con from con pool =>{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    return Ok(db_con);
}
