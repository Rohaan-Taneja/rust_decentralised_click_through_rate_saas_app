use std::sync::Arc;
use axum::{
    Extension, Router, extract::Path,routing::{post}
};


use crate::{
    AppState,

    errors::PersErrors,
 services::users::{check_user, generate_presigned_url},
};

pub fn user_handler() -> Router {
    Router::new()
        .route("/sign-in", post(sign_in))
        .route("/create-url/{file_size}", post(gen_url))
}

/**
 * it will authenticate the user and give jwt tokens to user after signing the message
 */
pub async fn sign_in(Extension(app_state): Extension<Arc<AppState>>) -> Result<(), PersErrors> {

    // 1) we will get wallet address + signed hash for validation this is the user who has connected his wallet
    // 2) we will search do we have any existing wallet in our db 
    // 3) if yes , then we will return jwt token 
    // 4) if no , then we will save user , create jwt token and return it to user 


    let db_pool = app_state.db_pool.clone();


    

    let acc_add = "6tyVk25iuv7fXUKCTbUmuv2XTDLP1ifQbXTBeFdVuiUV";

    
    println!("we are heree before storing data");

    let  (is_existing_user , user)  = check_user(&db_pool, acc_add).await?;

    println!("\n \n  {} {:?}" , is_existing_user , user);

    if  !is_existing_user {


    }
    // create jwt token and return 


    Ok(())

    // todo , incomplete logic , just doing the harcoded account , no verifying f message now , will do it later
}

pub async fn gen_url(Path(file_size) : Path<i64>) -> Result<() , PersErrors> {

    // 1) we will get wallet address + signed hash for validation this is the user who has connected his wallet
    // 2) we will search do we have any existing wallet in our db 
    // 3) if yes , then we will return jwt token 
    // 4) if no , then we will save user , create jwt token and return it to user 
    
    println!("we are heree before storing data");

    let  data = generate_presigned_url(file_size).await?;

    println!("\n \n {:?}" , data);


    Ok(())
}
