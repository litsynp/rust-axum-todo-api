use serde::{Deserialize, Serialize};

use crate::todo::models::Todo;

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct TodoView {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Todo> for TodoView {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            completed: todo.completed,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewTodoRequest {
    pub title: String,
    pub description: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct EditTodoRequest {
    pub title: String,
    pub description: String,
    pub completed: bool,
}
