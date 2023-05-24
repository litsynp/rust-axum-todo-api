use serde::{Deserialize, Serialize};

use crate::user::models::User;

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct UserView {
    pub id: i32,
    pub email: String,
    pub nickname: String,
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            nickname: user.nickname,
        }
    }
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewUserRequest {
    pub email: String,
    pub password: String,
    pub nickname: String,
}
