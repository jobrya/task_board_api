use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub task_id: u32,
    pub task_group_id: u32,
    pub text: String,
}