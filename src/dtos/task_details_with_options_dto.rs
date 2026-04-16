use serde::Serialize;

use crate::models::task_options::TaskOption;

#[derive(Serialize)]
pub struct TaskDetailsWithOptionsDTO {
    pub task_id: String,
    pub task_options: Vec<TaskOption>,
}

impl TaskDetailsWithOptionsDTO {
    pub fn from(task_complete_data: (String, Vec<TaskOption>)) -> TaskDetailsWithOptionsDTO {
        return TaskDetailsWithOptionsDTO {
            task_id: task_complete_data.0,
            task_options: task_complete_data.1,
        };
    }
}
