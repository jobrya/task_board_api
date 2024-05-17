use super::task_group::TaskGroup;
use super::task::Task;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub board_id: i32,
    pub account_id: i32,
    pub task_groups: Vec<TaskGroup>,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize)]
pub struct BoardRequest {
    pub board_id: i32,
    pub account_id: i32,
}

impl Board {
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(&self)
            .expect("failed to convert Board to json string");
        return json.to_string();
    }
}