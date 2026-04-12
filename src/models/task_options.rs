use crate::schema::task_options;
use diesel::Selectable;
use diesel::pg::Pg;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable , Debug , Serialize , Deserialize)]
#[diesel(table_name = task_options)]
#[diesel(check_for_backend(Pg))]
pub struct TaskOption {
    pub id: Uuid,
    pub task_id: Uuid,
    pub image_url: String,
    pub no_of_times_image_selected: i16,
}

#[derive(Insertable)]
#[diesel(table_name = task_options)]
pub struct NewTaskOption {
    pub task_id: Uuid,
    pub image_url: String,

    pub no_of_times_image_selected: i16,
}
