use axum::{Extension, Router, extract::Path, middleware, routing::post};
use std::sync::Arc;

use crate::{
    AppState, errors::PersErrors, middlewares::auth_middleware::authenticate_user, models::user::UserTypeEnum, services::users::{generate_presigned_url, get_or_create_user}, structs::EncodedUserData, utils::jwt::encode_user_info
};

pub fn user_handler() -> Router {
    Router::new()
        .route("/sign-in", post(sign_in))
        .merge(protected_route())
}

pub fn protected_route() -> Router {
    Router::new()
        .route("/create-url/{file_size}", post(gen_url))
        .layer(middleware::from_fn(authenticate_user)) // route with auth middleware
}

/**
 * it will authenticate the user and give jwt tokens to user after signing the message
 */
pub async fn sign_in(Extension(app_state): Extension<Arc<AppState>> , ) -> Result<String, PersErrors> {

    // 1) we will get wallet address + signed hash for validation this is the user who has connected his wallet
    // 2) we will search do we have any existing wallet in our db
    // 3) if yes , then we will return jwt token
    // 4) if no , then we will save user , create jwt token and return it to user
    let db_pool = app_state.db_pool.clone();

    // todo =>  nounce , wallet_address , user_type
    let user_wallet_address = "6tyVk25iuv7fXUKCTbUmuv2XTDLP1ifQbXTBeFdVuiUVwq";

    let user_type = UserTypeEnum::WORKER;

    println!("we are here before storing data");

    // function to check if user is existing user or we have to initialized this user 
    let user = get_or_create_user(user_wallet_address, user_type ,  db_pool).await?;

    // creating jwt token for user
    let token = encode_user_info(user.id, user_wallet_address.to_owned() , user_type)?;

    println!("{}", token);

    Ok(token)
}


/**
 * aws url genrator to post image 
 */
pub async fn gen_url(
    Extension(user_data): Extension<EncodedUserData>,
    Path(file_size): Path<i64>,
) -> Result<String, PersErrors> {
    

    println!("we are heree before storing data , user id {}", user_data.user_id);

    let s3_pre_signed_url = generate_presigned_url(file_size).await?;

    println!("\n \n {:?}", s3_pre_signed_url);

    Ok(s3_pre_signed_url)
}
