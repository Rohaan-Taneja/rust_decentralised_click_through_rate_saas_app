use std::{env, time::Duration};

use aws_config::BehaviorVersion;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::http::StatusCode;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper, result::Error::NotFound};
use diesel_async::RunQueryDsl;

use crate::{
    DbPool,
    db::get_connection_from_pool,
    errors::PersErrors,
    models::user::{NewUser, UserStruct, UserTypeEnum},
    schema::users,
};

pub async fn get_or_create_user(
    user_wallet_address: &str,
    db_pool: DbPool,
) -> Result<UserStruct, PersErrors> {
    let mut db_con = get_connection_from_pool(&db_pool).await?;

    let (is_existing_user, mut user) = check_user(&db_pool, user_wallet_address).await?;

    if !is_existing_user {
        // create user
        user = Some(
            create_new_user(db_pool.clone(), user_wallet_address, UserTypeEnum::CREATOR).await?,
        );
    }

    match user {
        Some(u) => Ok(u),
        None => {
            return Err(PersErrors::new(
                "user is neitng present nor we are able to create it ",
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    }
}

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

/**
 * service to create a new user
 */
pub async fn create_new_user(
    db_pool: DbPool,
    user_wallet_address: &str,
    user_type: UserTypeEnum,
) -> Result<UserStruct, PersErrors> {
    let mut db_con = get_connection_from_pool(&db_pool).await?;

    let new_user = NewUser {
        user_wallet_address: user_wallet_address.to_owned(),
        userr_type: user_type,
    };

    let user = diesel::insert_into(users::table)
        .values(new_user)
        .returning(UserStruct::as_returning())
        .get_result(&mut db_con)
        .await
        .map_err(|e| {
            PersErrors::new(
                format!("new user insertion failed => {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    return Ok(user);
}



/**
 * service to genrate a presigned url in s3 bucket 
 * we will get size of the file and we will see if it less than 2 mb , then we will creat a url to store that data in the s3
 * we will return this url to frontend and then frontend will directly upload the image to this url and of this size (put req , when size is defined)
 * url has expiry , size , content_type restriction
 */
pub async fn generate_presigned_url(file_size: i64) -> Result<String, PersErrors> {
    // max image size in bytes
    let max_file_size = 1024 * 1024 * 2;

    if &file_size > &max_file_size {
        return Err(PersErrors::new(
            "file size cannot be greater that 2 mb ",
            StatusCode::FORBIDDEN,
        ));
    }
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let s3 = aws_sdk_s3::Client::new(&config);

    println!("file size = {}", &file_size);
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
