use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub nickname: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
