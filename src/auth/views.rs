use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenView {
    pub user_id: i32,
    pub access_token: String,
    pub refresh_token: String,
}
