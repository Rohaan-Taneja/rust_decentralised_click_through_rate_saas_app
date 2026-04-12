use crate::schema::sql_types::TaskStatus;
use crate::schema::tasks;
use chrono::{DateTime, Utc};
use diesel::{Selectable};
use diesel::pg::Pg;
use diesel::prelude::{Insertable, Queryable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DbEnum, Debug, Clone, Copy , Serialize , Deserialize)]
#[ExistingTypePath = "TaskStatus"]
pub enum TaskUpdates {
    #[db_rename = "ACTIVE"]
    ACTIVE,
    #[db_rename = "COMPLETED"]
    COMPLETED,
}

#[derive(Queryable, Selectable ,Serialize , Deserialize, Debug)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(Pg))]
pub struct Task {
    pub id: Uuid,
    pub user_wallet_address: String,
    pub txn_hash: String,
    pub is_active: TaskUpdates,
    pub no_of_times_taks_done: i16,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub user_wallet_address: String,
    pub txn_hash: String,
    pub is_active: TaskUpdates,
    pub no_of_times_taks_done: i16,
}
