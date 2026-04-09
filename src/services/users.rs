use std::{env, time::Duration};

use aws_config::BehaviorVersion;
use aws_sdk_s3::presigning::{ PresigningConfig};
use axum::{ http::StatusCode};
use diesel::{ExpressionMethods, query_dsl::methods::FilterDsl, result::Error::NotFound};
use diesel_async::RunQueryDsl;

use crate::{
    DbPool, db::get_connection_from_pool, errors::PersErrors, models::user::UserStruct,
    schema::users,
};

/**
 * function to check if we have a user or not
 * result => it will return bool + user(if already present , else null)
 */
pub async fn check_user(
    db_pool: &DbPool,
    user_wallet_address: &str,
) -> Result<(bool, Option<UserStruct>), PersErrors> {
    let mut db_con = get_connection_from_pool(db_pool).await?;

    // .await here — truly non-blocking!
    let result = users::table
        .filter(users::user_wallet_address.eq(user_wallet_address))
        .first::<UserStruct>(&mut db_con)
        .await;

    match result {
        Ok(user_data) => Ok((true, Some(user_data))),
        Err(e) => {
            if e == NotFound {
                Ok((false, None))
            } else {
                return Err(PersErrors::new(
                    format!("error fetching user {}: {}", user_wallet_address, e),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        }
    }
}

pub async fn generate_presigned_url(file_size :i64 ) -> Result<String, PersErrors> {

    // max image size in bytes
    let max_file_size = 1024*1024*2;

    if &file_size > &max_file_size {
        return Err(PersErrors::new("file size cannot be greater that 2 mb ", StatusCode::FORBIDDEN))
    }
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let s3 = aws_sdk_s3::Client::new(&config);

    println!("file size = {}" , &file_size);
    // getting bucket
    let bucket = env::var("AWS_BUCKET").map_err(|e| {
        PersErrors::new(
            format!("getting error while fetching bucket name from env {}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    // getting folder in bucket where all the images will be stored
    // eg => bucket/ctr_images , then we will create folder for each user => bucket/key/{user_id}/random_number
    let key = env::var("AWS_BUCKET_KEY").map_err(|e| {
        PersErrors::new(
            format!(
                "getting error while fetching bucket name key from env {}",
                e
            ),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    
    println!("setup created");
    // with size , use post , else put
    let presigned_url = s3
        .put_object()
        .bucket(bucket)
        .key(format!("{}/user_image_uploaded", key))
        // .content_length(file_size)    // uncomment it , when we will sending size from the frontend , a little diff also rejects the upload
        .presigned(
            PresigningConfig::builder()
                .expires_in(Duration::from_secs(60 * 5))
                .build()
                .map_err(|e| PersErrors::new(e.to_string(), StatusCode::FAILED_DEPENDENCY))?,
        )
        .await
        .map_err(|e| {
            PersErrors::new(
                format!("presigned_error {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;


    Ok(presigned_url.uri().to_string())
}
