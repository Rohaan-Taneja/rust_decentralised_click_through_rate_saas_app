use axum::{
    Extension, Json, Router, extract::Path, http::StatusCode, middleware, response::IntoResponse,
    routing::{post , get},
};
use serde::{Deserialize, Serialize};
use std::{fmt::Error, sync::Arc};

use crate::{
    AppState,
    dtos::{naunce_auth_dto::NaunceAuthStruct, sign_in_user_dto::SignInUserDTO},
    errors::PersErrors,
    middlewares::{
        auth_middleware::authenticate_user, user_type_middleware::creator_validator_middleware,
    },
    models::user::UserTypeEnum,
    services::users::{
        generate_naunce_data_for_autheticate_account, generate_presigned_url, get_or_create_user,
        verify_user_wallet_signature,
    },
    structs::EncodedUserData,
    utils::jwt::encode_user_info,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct S3ImageData {
    url: String,
    img_id: String,
    user_id: String,
}

pub fn user_handler() -> Router {
    Router::new()
        .route("/create-naunce/{user_wallet_address}", get(create_naunce_for_user))
        .route("/sign-in", post(sign_in))
        .merge(protected_route())
}

pub fn protected_route() -> Router {
    Router::new()
        .route("/create-url/{file_size}", post(gen_url))
        .layer(middleware::from_fn(creator_validator_middleware))
        .layer(middleware::from_fn(authenticate_user)) // route with auth middleware
}


/**
 * creaet naunce , store it in redis for 5 mins and return nanuce to user 
 */
pub async fn create_naunce_for_user(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(user_wallet_address): Path<String>,
) -> Result<impl IntoResponse, PersErrors> {
    let r_pool = app_state.redis_pool.clone();
    let res = generate_naunce_data_for_autheticate_account(r_pool, &user_wallet_address).await?;

    Ok(Json(NaunceAuthStruct::from(res)))
}

/**
 * it will get signed message from user wallet
 * we will validate the signed message , naunce from redis
 * and delete the naunce from redis 
 */
pub async fn sign_in(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(signin_data): Json<SignInUserDTO>,
) -> Result<impl IntoResponse, PersErrors> {
    // 1) we will get wallet address + signed hash for validation this is the user who has connected his wallet
    // 2) we will search do we have any existing wallet in our db
    // 3) if yes , then we will return jwt token
    // 4) if no , then we will save user , create jwt token and return it to user

    println!("this is the input {signin_data:?}");

    let r_pool = app_state.redis_pool.clone();

     match verify_user_wallet_signature(r_pool , signin_data.sign.to_owned(), signin_data.publickey.to_owned() , signin_data.naunce).await {
        Ok(ans) => {
            if !ans {
                return Err(PersErrors::new(
                    "wallet cannot be verified",
                    StatusCode::BAD_REQUEST,
                ));
            }
        }
        Err(e) => return Err(e),
    }

    let db_pool = app_state.db_pool.clone();

    

    println!("we are here before storing data {:?}" , signin_data);

    // function to check if user is existing user or we have to initialized this user
    let user = get_or_create_user(&signin_data.publickey, signin_data.user_type, db_pool).await?;

    // creating jwt token for user
    let token = encode_user_info(user.id, signin_data.publickey.to_owned(), signin_data.user_type)?;

    println!("{}", token);

    Ok(Json(token))
}

/**
 * aws url genrator to post image
 */
pub async fn gen_url(
    Extension(user_data): Extension<EncodedUserData>,
    Path(file_size): Path<i64>,
) -> Result<impl IntoResponse, PersErrors> {
    println!(
        "we are heree before storing data , user id {}",
        user_data.user_id
    );

    let s3_image_data = generate_presigned_url(file_size, user_data.user_id.to_string()).await?;

    println!("\n \n {:?}", s3_image_data);

    Ok(Json(S3ImageData {
        url: s3_image_data.1,
        img_id: s3_image_data.0,
        user_id: user_data.user_id.to_string(),
    }))
}
