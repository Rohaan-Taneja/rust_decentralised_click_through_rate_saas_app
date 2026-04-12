use serde::Deserialize;


use validator::{Validate};


#[derive(Debug , Validate , Deserialize)]
pub struct NewTaskDTO{
    pub images : Vec<String>,
    pub txn_sign : String


}