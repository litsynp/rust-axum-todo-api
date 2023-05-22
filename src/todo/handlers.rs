use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use sqlx::PgPool;

use crate::common::pagination::PaginationParams;
use crate::todo::models::{EditTodo, NewTodo};
use crate::todo::repository;

pub async fn create_todo(
    Extension(pool): Extension<PgPool>,
    Json(todo): Json<NewTodo>,
) -> impl IntoResponse {
    let todo = repository::create_todo(pool, todo)
        .await
        .expect("Failed to create todo");

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todos(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<PaginationParams>,
) -> impl IntoResponse {
    let (page, limit) = (query.page.unwrap_or(1), query.limit.unwrap_or(10));

    let todos = repository::find_todos(pool, page, limit)
        .await
        .expect("Failed to retrieve todos from database");

    (StatusCode::OK, Json(json!({ "todos": todos })))
}

pub async fn find_todo_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let todo = repository::find_todo_by_id(pool, id)
        .await
        .expect("Failed to retrieve todo from database");

    (StatusCode::OK, Json(todo))
}

pub async fn edit_todo_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(todo): Json<EditTodo>,
) -> impl IntoResponse {
    let todo = repository::edit_todo(pool, id, todo)
        .await
        .expect("Failed to edit todo");

    (StatusCode::OK, Json(todo))
}

pub async fn delete_todo_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    repository::delete_todo_by_id(pool, id).await;

    StatusCode::NO_CONTENT
}
