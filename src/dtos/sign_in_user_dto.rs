use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::UserTypeEnum;


#[derive(Debug , Serialize , Deserialize)]
pub struct SignInUserDTO {
    pub sign : Vec<u8> ,
    pub publickey : String,
    pub user_type : UserTypeEnum,
    pub naunce : Uuid
}