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