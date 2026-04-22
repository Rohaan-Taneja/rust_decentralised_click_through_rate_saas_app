use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::structs::NaunceAuth;

#[derive(Serialize , Deserialize , Debug)]
pub struct NaunceAuthStruct {
    pub naunce: Uuid,
}

impl NaunceAuthStruct {
    pub fn from(one_time_auth_data: NaunceAuth) -> NaunceAuthStruct {
        return NaunceAuthStruct {
            naunce: one_time_auth_data.naunce,
        };
    }
}
