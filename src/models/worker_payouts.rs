

use crate::schema::{sql_types::PayoutStatus,  payouts};
use chrono::{DateTime, Utc};
use diesel::{Selectable};
use diesel::pg::Pg;
use diesel::prelude::{Insertable, Queryable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DbEnum, Debug , Serialize , Deserialize)]
#[ExistingTypePath = "PayoutStatus"]
pub enum WorkerPayoutStatus {
    #[db_rename = "PENDING"]
    PENDING,
    #[db_rename = "COMPLETED"]
    COMPLETED,
    #[db_rename = "FAILED"]
    FAILED,
}


#[derive(Queryable, Selectable ,Serialize , Deserialize, Debug)]
#[diesel(table_name = payouts)]
#[diesel(check_for_backend(Pg))]
pub struct Payouts {
    pub id: Uuid,
    pub worker_wallet_address: String,
    pub amount: String ,
    pub created_at:  DateTime<Utc> ,
    pub completed_at: Option<DateTime<Utc>>,
    pub txn_hash: String,
    pub payout_status : WorkerPayoutStatus

}

pub struct NewPayout {
     pub worker_wallet_address: String,
    pub amount: String ,
    pub txn_hash: String,
    pub payout_status : WorkerPayoutStatus
}