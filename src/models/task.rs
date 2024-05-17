use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub task_id: i32,
    pub task_group_id: i32,
    pub task_text: String,
}

#[derive(Deserialize)]
pub struct TaskRequest {
    pub task_group_id: i32,
    pub task_text: String,
}