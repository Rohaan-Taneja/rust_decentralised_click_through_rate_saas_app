// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pay_status"))]
    pub struct PayStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payout_status"))]
    pub struct PayoutStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "task_status"))]
    pub struct TaskStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_type"))]
    pub struct UserType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PayoutStatus;

    payouts (id) {
        id -> Uuid,
        worker_wallet_address -> Varchar,
        amount -> Varchar,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        txn_hash -> Varchar,
        payout_status -> PayoutStatus,
    }
}

diesel::table! {
    submission_details (id) {
        id -> Uuid,
        worker_wallet_address -> Varchar,
        task_id -> Uuid,
        selected_option_id -> Uuid,
    }
}

diesel::table! {
    task_options (id) {
        id -> Uuid,
        task_id -> Uuid,
        image_url -> Varchar,
        no_of_times_image_selected -> Int2,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TaskStatus;

    tasks (id) {
        id -> Uuid,
        user_wallet_address -> Varchar,
        txn_hash -> Varchar,
        is_active -> TaskStatus,
        no_of_times_taks_done -> Int2,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserType;

    users (id) {
        id -> Uuid,
        user_wallet_address -> Varchar,
        userr_type -> UserType,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PayStatus;

    worker_work_details (id) {
        id -> Uuid,
        worker_wallet_address -> Varchar,
        total_lifetime_tasks -> Int4,
        current_no_of_tasks_for_payout -> Int4,
        payout_status -> PayStatus,
        txn_hash_of_withdrawal -> Nullable<Varchar>,
    }
}

diesel::joinable!(submission_details -> task_options (selected_option_id));
diesel::joinable!(submission_details -> tasks (task_id));
diesel::joinable!(task_options -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    payouts,
    submission_details,
    task_options,
    tasks,
    users,
    worker_work_details,
);
