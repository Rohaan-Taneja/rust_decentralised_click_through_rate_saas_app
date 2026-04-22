use std::{env, str::FromStr, time::Duration};

use aws_config::BehaviorVersion;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::http::StatusCode;
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper, result::Error::NotFound};
use diesel_async::{AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{
    DbPool,
    db::get_connection_from_pool,
    errors::PersErrors,
    models::{
        user::{NewUser, UserStruct, UserTypeEnum},
        worker_work_details::{NewOrUpdateWorkerWorkDetails, PaymentStatus, WorkerWorkDetails},
    },
    redis::{RedisPool, get_redis_conn_from_pool},
    schema::{users, worker_work_details},
    structs::NaunceAuth,
};
use solana_sdk::{pubkey::Pubkey, signature::Signature};

pub async fn get_or_create_user(
    user_wallet_address: &str,
    user_type: UserTypeEnum,
    db_pool: DbPool,
) -> Result<UserStruct, PersErrors> {
    let (is_existing_user, mut user) = check_user(&db_pool, user_wallet_address, user_type).await?;

    if !is_existing_user {
        // create user
        user = Some(create_new_user(db_pool.clone(), user_wallet_address, user_type).await?);
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
    user_type: UserTypeEnum,
) -> Result<(bool, Option<UserStruct>), PersErrors> {
    let mut db_con = get_connection_from_pool(db_pool).await?;

    let result = users::table
        .filter(users::user_wallet_address.eq(user_wallet_address))
        .filter(users::userr_type.eq(user_type))
        .first::<UserStruct>(&mut db_con)
        .await;

    match result {
        Ok(user_data) => Ok((true, Some(user_data))),
        Err(e) => {
            if e == NotFound {
                println!("\n \n we did not found any user \n \n \n {e}");
                Ok((false, None))
            } else {
                println!("\n \n we are getting error while finding the user \n \n \n {e}");
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
 * and if user is of type WORKER , we will create/initialize its worker_work_details also
 */
pub async fn create_new_user(
    db_pool: DbPool,
    user_wallet_address: &str,
    user_type: UserTypeEnum,
) -> Result<UserStruct, PersErrors> {
    let mut __db_con__ = get_connection_from_pool(&db_pool).await?;

    // we will creat a txn , if user type == worker , then we will create worker details row

    let user = __db_con__
        .transaction::<_, diesel::result::Error, _>(|__db_con__| {
            async move {
                let new_user = NewUser {
                    user_wallet_address: user_wallet_address.to_owned(),
                    userr_type: user_type,
                };

                let user = diesel::insert_into(users::table)
                    .values(new_user)
                    .returning(UserStruct::as_returning())
                    .get_result::<UserStruct>(__db_con__)
                    .await?;

                println!("this is the new user creates {:?}" , user);

                // if new user is a worker , we will create its worker work details also
                match user.userr_type {
                    UserTypeEnum::CREATOR => {}
                    UserTypeEnum::WORKER => {
                        // create a worker worok details of the user

                        let new_worker_work_details = NewOrUpdateWorkerWorkDetails {
                            worker_wallet_address: user_wallet_address.to_owned(),
                            total_lifetime_tasks: 0,
                            current_no_of_tasks_for_payout: 0,
                            payout_status: PaymentStatus::NOT_ELIGIBLE,
                            txn_hash_of_withdrawal: None,
                        };

                        let worker = diesel::insert_into(worker_work_details::table)
                            .values(&new_worker_work_details)
                            .returning(WorkerWorkDetails::as_returning())
                            .get_result(__db_con__)
                            .await?;
                    }
                }

                Ok(user)
            }
            .scope_boxed()
        })
        .await
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => PersErrors::new(
                format!(
                    "wallet address {} already exists as a {:?}",
                    user_wallet_address, user_type
                ),
                StatusCode::CONFLICT,
            ),
            _ => PersErrors::new(
                format!("new user insertion failed => {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        })?;

    Ok(user)
}

/**
 * service to genrate a presigned url in s3 bucket
 * we will get size of the file and we will see if it less than 2 mb , then we will creat a url to store that data in the s3
 * we will return this url to frontend and then frontend will directly upload the image to this url and of this size (put req , when size is defined)
 * url has expiry , size , content_type restriction
 */
pub async fn generate_presigned_url(
    file_size: i64,
    user_id: String,
) -> Result<(String, String), PersErrors> {
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

    let img_id = Uuid::new_v4().to_string();

    // with size , use post , else put
    let presigned_url = s3
        .put_object()
        .bucket(bucket)
        .key(format!("{}/{}/{}", key, user_id, img_id))
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

    Ok((img_id, presigned_url.uri().to_string()))
}

/**
 * we are using naunce , so that on every autheticating time user will sign a unique message
 * inputs => user wallet address and redis pool for temp(5 mins) storing unique (user_naunce and exp and wallet and used:bool) details
 * result ->
 *  store data in redis
 *  and return naunce in the return
 */
pub async fn generate_naunce_data_for_autheticate_account(
    r_pool: RedisPool,
    user_wallet_address: &str,
) -> Result<NaunceAuth, PersErrors> {
    let mut redis_con = get_redis_conn_from_pool(&r_pool).await?;

    let unique_naunce = Uuid::new_v4();

    let naunce_struct = NaunceAuth {
        naunce: unique_naunce,
        user_wallet_addrss: user_wallet_address.to_owned(),
        used: false,
        exp: Utc::now() + Duration::from_secs(300),
    };

    let json_naunce = serde_json::to_string(&naunce_struct).map_err(|e| {
        PersErrors::new(
            format!("error while converting naunce to json => ${}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    // this redis saved naunce will be deleted after 300 sec..
    // exp is 5 mins
    let _: () = redis_con
        .set_ex(format!("naunce:${}", unique_naunce), json_naunce , 300)
        .await
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(naunce_struct)
}

/**
 * function to verify user signed message
 * result =>
 * we will get naunce , sign and user wlalet
 * we will find naunce from redis
 * and validate , exp and wallet address
 * reconstructing the message and veryfying the message
 * deleting the redis naunce after veryfying
 */
pub async fn verify_user_wallet_signature(
    redis_pool: RedisPool,
    message_signature: Vec<u8>,
    publickey: String,
    naunce: Uuid,
) -> Result<bool, PersErrors> {
    let mut redis_con = get_redis_conn_from_pool(&redis_pool).await?;

    let n = format!("naunce:${}", naunce);

    // getting naunce data from redis
    let string_naunce_struct: String = redis_con
        .get(n.to_owned())
        .await
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::BAD_REQUEST))?;
    let naunce_data: NaunceAuth = serde_json::from_str(&string_naunce_struct)
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    // checking expiry of naunce + signature
    if Utc::now() > naunce_data.exp {
        return Err(PersErrors::new(
            "signature have been expired , please validte again",
            StatusCode::BAD_REQUEST,
        ));
    }

    // if adrress stored in naucne is not equal to for whic this nanucne is created , we will throw error
    if publickey != naunce_data.user_wallet_addrss {
        return Err(PersErrors::new(
            "wallet address mismatch",
            StatusCode::UNAUTHORIZED,
        ));
    }

    println!(
        "this is the redis stored and converted {} {:?}",
        string_naunce_struct, naunce_data
    );

    // reconstructing the message
    let message = format!(
        "sign this message to authenticate with CTR Checker: {}",
        &naunce
    );

    println!("\n \n this is the message {} \n " , message);

    // constructing pub key in which the verify function wants
    let pub_key = Pubkey::from_str(&publickey)
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    // converting signature into 64 bytes fixed sized array
    let sign_array: [u8; 64] = message_signature.try_into().map_err(|_| {
        PersErrors::new(
            "signature must be of 64 bytes array",
            StatusCode::BAD_REQUEST,
        )
    })?;

    let sign: Signature = Signature::from(sign_array);

    let is_verified = sign.verify(pub_key.as_ref(), message.as_bytes());

    println!(" \n \n. \n This is the sign result {}" , is_verified);

    // deleting redis , after validating
    if is_verified {
        let _: () = redis_con
            .del(n)
            .await
            .map_err(|e| PersErrors::new(e.to_string(), StatusCode::BAD_REQUEST))?;
    }

    Ok(is_verified)
}
