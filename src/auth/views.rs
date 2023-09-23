use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct LoginRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TokenView {
    pub user_id: i32,
    pub access_token: String,
    pub refresh_token: String,
}
