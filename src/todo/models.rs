use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
