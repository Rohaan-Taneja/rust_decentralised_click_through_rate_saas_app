use crate::schema::sql_types::UserType;
use crate::schema::users;
use diesel::{pg::Pg, prelude::*};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(DbEnum, Debug, Clone, Copy)]
#[ExistingTypePath = "UserType"]
pub enum UserTypeEnum {
    CREATOR,
    WORKER,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: Uuid,
    pub user_wallet_address: String,
    pub userr_type: UserTypeEnum,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_wallet_address: String,
    pub userr_type: UserTypeEnum,
}