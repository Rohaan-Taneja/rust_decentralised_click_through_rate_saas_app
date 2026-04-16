use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::task::{Task, TaskUpdates};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TaskDTO {
    pub id: Uuid,
    pub is_active: TaskUpdates,
    pub no_of_times_task_done: i16,
    pub created_at: DateTime<Utc>,
}

impl TaskDTO {
    pub fn to_tasks(vec_of_taks: Vec<Task>) -> Vec<TaskDTO> {
        let ans = vec_of_taks.into_iter().map(TaskDTO::to_task).collect();

        return ans;
    }

    pub fn to_task(task_struct: Task) -> TaskDTO {
        return TaskDTO {
            id: task_struct.id,
            is_active: task_struct.is_active,
            no_of_times_task_done: task_struct.no_of_times_taks_done,
            created_at: task_struct.created_at,
        };
    }
}
