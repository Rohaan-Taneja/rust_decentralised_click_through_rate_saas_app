use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize , Deserialize , Clone, Copy , Debug)]
pub struct TaskSubmissionDataDTO {
    pub task_id : Uuid ,
    pub selection_option_id : Uuid

}