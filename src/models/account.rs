#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Account {
    pub account_id: i32,
    pub username: String,
}

impl Account {
    pub fn to_json(&self) -> String {
    let json = serde_json::to_string(&self)
        .expect("failed to convert Account to json string");
    return json.to_string();
    }
}

#[derive(serde::Deserialize)]
pub struct AccountRequest {
    pub username: String,
    pub password: String,
}