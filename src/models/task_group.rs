use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TaskGroup {
    pub task_group_id: u32,
    pub board_id: u32,
    pub group_name: String,
}