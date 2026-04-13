use crate::schema::users;
use diesel::{pg::Pg, prelude::*};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone, Copy)]
#[ExistingTypePath = "crate::schema::sql_types::UserType"]
pub enum UserTypeEnum {
    #[db_rename = "CREATOR"]
    CREATOR,
    #[db_rename = "WORKER"]
    WORKER,
}

#[derive(Queryable, Selectable ,Deserialize, Serialize , Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct UserStruct {
    pub id: Uuid,
    pub user_wallet_address: String,
    pub userr_type: UserTypeEnum,
}

#[derive(Insertable , Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_wallet_address: String,
    pub userr_type: UserTypeEnum,
}