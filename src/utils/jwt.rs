// function to encrypt user_id + token in jwt token

use std::{env, time::Duration};

use axum::http::StatusCode;

use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::PersErrors, models::user::UserTypeEnum, structs::EncodedUserData};




pub fn encode_user_info(
    user_id: Uuid,
    user_wallet_address: String,
    user_type : UserTypeEnum
) -> Result<String, PersErrors> {
    let user_data = EncodedUserData {
        user_id,
        user_wallet_address,
        user_type,
        exp : (Utc::now() + Duration::from_hours(24)).timestamp() as usize
    };



    let jwt_secret = env::var("JWT_SECRET").map_err(|e| PersErrors::new(format!("error in fetching jwt secret {}" , e) , StatusCode::INTERNAL_SERVER_ERROR))?;

    let token = encode(&Header::default(), &user_data, &EncodingKey::from_secret(jwt_secret.as_bytes())).map_err(|e| PersErrors::new(format!("gettig error in creating auth token => {e}") , StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(token)
}

// function to decrypt/verify incoming jwt user token

pub fn verify_user( jwt_token : String)-> Result<EncodedUserData , PersErrors>{

    println!("{}" , jwt_token);

    let jwt_secret = env::var("JWT_SECRET").map_err(|e| PersErrors::new(format!("error in fetching jwt secret {}" , e) , StatusCode::INTERNAL_SERVER_ERROR))?;

    println!("secret {}" , jwt_secret);

    // the default validation checks experiy also from claims.exp value
    let details = match decode::<EncodedUserData>(&jwt_token, &DecodingKey::from_secret(jwt_secret.as_bytes()), &Validation::default()) {

        Ok(d) => {
            d
        },
        Err(e) =>{
            println!("error in jwt {}" ,e );
            return Err(PersErrors::new(format!("user unauthorized {e}") , StatusCode::UNAUTHORIZED) )
           
        }


    };
    
    println!("this is the ans {:?}" , details);
    let user_details = details.claims;

    println!("we are here {:?}" , user_details);
    
    Ok(user_details)

}
