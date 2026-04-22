use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::UserTypeEnum;

#[derive(Serialize , Deserialize , Debug , Clone)]
pub struct EncodedUserData {
    pub user_id: Uuid,
    pub user_wallet_address: String,
    pub user_type : UserTypeEnum ,
    pub exp : usize
}


/**
 * we are storing 1 time use , unique data for user to signing message for wallet authetication
 * 
 */
#[derive(Serialize , Deserialize , Debug , Clone)]
pub struct NaunceAuth{
 pub naunce : Uuid ,
 pub user_wallet_addrss : String ,
 pub used : bool ,
 pub exp : DateTime<Utc>

}