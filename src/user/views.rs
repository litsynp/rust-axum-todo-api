use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::user::models::User;

#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserView {
    pub id: i32,
    #[schema(example = "user@example.com")]
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

#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewUserRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    pub nickname: String,
    pub password: String,
}
