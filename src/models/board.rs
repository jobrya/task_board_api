use super::task_group::TaskGroup;
use super::task::Task;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub board_id: u32,
    pub account_id: u32,
    pub task_groups: Vec<TaskGroup>,
    pub tasks: Vec<Task>,
}

impl Board {
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(&self)
            .expect("failed to convert Board to json string");
        return json.to_string();
    }
}