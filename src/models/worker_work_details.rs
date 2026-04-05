use crate::schema::sql_types::PayStatus;
use crate::schema::worker_work_details;
use diesel::Selectable;
use diesel::pg::Pg;
use diesel::prelude::{Insertable, Queryable};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(DbEnum, Debug, Clone, Copy)]
#[ExistingTypePath = "PayStatus"]
pub enum PaymentStatus {
    LOCKED,
    NOT_ELIGIBLE,
    CAN_WITHDRAW
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = worker_work_details)]
#[diesel(check_for_backend(Pg))]
pub struct Task {
    pub id: Uuid,
    pub worker_wallet_address: String,
    pub total_lifetime_tasks : i32 ,
    pub current_no_of_tasks_for_payout : i32 ,
    pub payout_status : PaymentStatus ,
    pub txn_hash_of_withdrawal: Option<String>,

}

#[derive(Insertable)]
#[diesel(table_name = worker_work_details)]
pub struct NewOrUpdateWorkerWorkDetails {
    pub worker_wallet_address: String,
    pub total_lifetime_tasks : i32 ,
    pub current_no_of_tasks_for_payout : i32 ,
    pub payout_status : PaymentStatus ,
    pub txn_hash_of_withdrawal: Option<String>,
}
