use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use sqlx::PgPool;

use crate::common::pagination::PaginationParams;
use crate::errors::ApiError;
use crate::todo::{
    models::{EditTodo, NewTodo, Todo},
    repository,
};

pub async fn create_todo(
    Extension(pool): Extension<PgPool>,
    Json(todo): Json<NewTodo>,
) -> Result<Json<Todo>, ApiError> {
    let todo = repository::create_todo(pool, todo).await;

    match todo {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}

pub async fn find_todos(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<PaginationParams>,
) -> Result<Json<Vec<Todo>>, ApiError> {
    let (page, limit) = (query.page.unwrap_or(1), query.limit.unwrap_or(10));

    let todos = repository::find_todos(pool, page, limit).await;

    match todos {
        Ok(todos) => Ok(Json(todos)),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}

pub async fn find_todo_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, ApiError> {
    let todo = repository::find_todo_by_id(pool, id).await;

    match todo {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Err(ApiError::new_not_found(format!(
                "Todo with id {} not found",
                id
            ))),
            _ => Err(ApiError::new_internal(e.to_string())),
        },
    }
}

pub async fn edit_todo_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(todo): Json<EditTodo>,
) -> Result<Json<Todo>, ApiError> {
    let todo = repository::edit_todo(pool, id, todo).await;

    match todo {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Err(ApiError::new_not_found(format!(
                "Todo with id {} not found",
                id
            ))),
            _ => Err(ApiError::new_internal(e.to_string())),
        },
    }
}

pub async fn delete_todo_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<()>, ApiError> {
    let result = repository::delete_todo_by_id(pool, id).await;

    match result {
        Ok(_) => Ok(Json(())),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}
