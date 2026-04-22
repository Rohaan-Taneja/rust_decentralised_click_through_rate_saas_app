use crate::schema::sql_types::CreatorTxnStatus;
use crate::schema::{creators_payment};
use chrono::{DateTime, Utc};
use diesel::{Selectable};
use diesel::pg::Pg;
use diesel::prelude::{Insertable, Queryable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DbEnum, Debug, Clone, Copy , Serialize , Deserialize)]
#[ExistingTypePath = "CreatorTxnStatus"]
pub enum CreatorsTxnStatus {
    #[db_rename = "ACTIVE"]
    CREATED,
     #[db_rename = "FAILED"]
    FAILED,
    #[db_rename = "COMPLETED"]
    COMPLETED,
}

#[derive(Queryable, Selectable ,Serialize , Deserialize, Debug)]
#[diesel(table_name = creators_payment)]
#[diesel(check_for_backend(Pg))]
pub struct CreatorsPayment {
    pub id:  Uuid ,
    pub creator_wallet_address  :  String,
    pub txh_sign: Option<String>,
    pub payment_status : CreatorsTxnStatus,
    pub created_at : DateTime<Utc>,

}

#[derive(Insertable)]
#[diesel(table_name = creators_payment)]
pub struct NewPaymentIntent {
    pub creator_wallet_address  :  String,
    pub payment_status : CreatorsTxnStatus,
}
