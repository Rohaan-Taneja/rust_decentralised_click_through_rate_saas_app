use serde::{Deserialize, Serialize};



#[derive(Deserialize , Serialize)]
pub struct TaskIdDTO {
    pub task_id : String
}


impl TaskIdDTO {

    pub fn from( task_id : String)-> TaskIdDTO {
        return TaskIdDTO { task_id : task_id }
    }
}