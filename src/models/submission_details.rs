use crate::schema::submission_details;
use diesel::Selectable;
use diesel::pg::Pg;
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = submission_details)]
#[diesel(check_for_backend(Pg))]
pub struct Task {
    pub id: Uuid,
    pub worker_wallet_address: String,
    pub task_id: Uuid,
    pub selected_option_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = submission_details)]
pub struct NewSubmissionDetails {
    pub worker_wallet_address: String,
    pub task_id: Uuid,
    pub selected_option_id: Uuid,
}
