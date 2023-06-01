use serde::{Deserialize, Serialize};

use crate::user::models::User;

pub static JWT_SECRET: &str = "foobar";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: i32,
    pub token_type: String,
    pub exp: usize,
}

impl Claims {
    pub fn from_user(user: &User, token_type: &str, exp: usize) -> Self {
        Claims {
            sub: user.nickname.clone(),
            user_id: user.id,
            token_type: token_type.to_string(),
            exp,
        }
    }
}
