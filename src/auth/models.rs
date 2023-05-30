use serde::{Deserialize, Serialize};

use crate::user::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    user_id: i32,
    token_type: String,
    exp: usize,
}

impl Claims {
    pub fn from_user(user: &User, token_type: String, exp: usize) -> Self {
        Claims {
            sub: user.nickname.clone(),
            user_id: user.id,
            token_type,
            exp,
        }
    }
}
