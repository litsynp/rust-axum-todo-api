use sqlx::PgPool;

use crate::todo::views::EditTodoRequest;
use crate::todo::{
    repository as todo_repository,
    views::{NewTodoRequest, TodoView},
};

pub async fn create_todo(pool: PgPool, new_todo: NewTodoRequest) -> Result<TodoView, sqlx::Error> {
    let todo = todo_repository::create_todo(pool, new_todo).await?;

    Ok(TodoView::from(todo))
}

pub async fn find_todos(pool: PgPool, page: i32, limit: i32) -> Result<Vec<TodoView>, sqlx::Error> {
    let todos = todo_repository::find_todos(pool, page, limit).await?;

    Ok(todos.into_iter().map(TodoView::from).collect())
}

pub async fn find_todo_by_id(pool: PgPool, id: i32) -> Result<TodoView, sqlx::Error> {
    let todo = todo_repository::find_todo_by_id(pool, id).await?;

    Ok(TodoView::from(todo))
}

pub async fn edit_todo_by_id(
    pool: PgPool,
    id: i32,
    edited_todo: EditTodoRequest,
) -> Result<TodoView, sqlx::Error> {
    let todo = todo_repository::edit_todo_by_id(pool, id, edited_todo).await?;

    Ok(TodoView::from(todo))
}

pub async fn delete_todo_by_id(pool: PgPool, id: i32) -> Result<(), sqlx::Error> {
    todo_repository::delete_todo_by_id(pool, id).await?;

    Ok(())
}
