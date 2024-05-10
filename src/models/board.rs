use serde_json::json;

pub struct Board {
    pub board_id: u32,
    pub user_id: u32, 
}

impl Board {
    pub fn to_json(&self) -> String{
        let board = json!({
            "board_id": &self.board_id.to_string(),
            "user_id": &self.user_id.to_string(),
        });
        return format!("{}", board.to_string());
    }
}