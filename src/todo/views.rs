use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::todo::models::Todo;

#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema)]
pub struct TodoView {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Todo> for TodoView {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            completed: todo.completed,
            created_at: todo.created_at.to_rfc3339(),
            updated_at: todo.updated_at.to_rfc3339(),
        }
    }
}

#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema)]
pub struct NewTodoRequest {
    pub title: String,
    pub description: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema)]
pub struct EditTodoRequest {
    pub title: String,
    pub description: String,
    pub completed: bool,
}
