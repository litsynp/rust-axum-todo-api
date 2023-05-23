use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct EditTodo {
    pub title: String,
    pub description: String,
    pub completed: bool,
}
