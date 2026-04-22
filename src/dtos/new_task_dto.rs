use serde::Deserialize;


use uuid::Uuid;
use validator::{Validate};


#[derive(Debug , Validate , Deserialize)]
pub struct NewTaskDTO{
    pub images : Vec<String>,
    pub txn_sign : String,
    pub payment_id : Uuid


}