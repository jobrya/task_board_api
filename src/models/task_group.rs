use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TaskGroup {
    pub task_group_id: i32,
    pub board_id: i32,
    pub group_name: String,
}

#[derive(Deserialize)]
pub struct TaskGroupRequest {
    pub board_id: i32,
    pub group_name: String,
}