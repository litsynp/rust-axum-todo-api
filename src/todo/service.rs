use sqlx::PgPool;

use crate::todo::{
    models::Todo,
    repository as todo_repository,
    views::{EditTodoRequest, NewTodoRequest},
};

pub async fn create_todo(pool: PgPool, new_todo: NewTodoRequest) -> Result<Todo, sqlx::Error> {
    todo_repository::create_todo(pool, new_todo).await
}

pub async fn find_todos(pool: PgPool, page: i32, size: i32) -> Result<Vec<Todo>, sqlx::Error> {
    todo_repository::find_todos(pool, page, size).await
}

pub async fn find_todo_by_id(pool: PgPool, id: i32) -> Result<Todo, sqlx::Error> {
    todo_repository::find_todo_by_id(pool, id).await
}

pub async fn edit_todo_by_id(
    pool: PgPool,
    id: i32,
    edited_todo: EditTodoRequest,
) -> Result<Todo, sqlx::Error> {
    todo_repository::edit_todo_by_id(pool, id, edited_todo).await
}

pub async fn delete_todo_by_id(pool: PgPool, id: i32) -> Result<(), sqlx::Error> {
    todo_repository::delete_todo_by_id(pool, id).await
}
