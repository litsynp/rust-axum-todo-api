use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use sqlx::PgPool;

use crate::common::{errors::ApiError, pagination::PaginationParams};
use crate::todo::{
    service as todo_service,
    views::{EditTodoRequest, NewTodoRequest, TodoView},
};

pub async fn create_todo(
    Extension(pool): Extension<PgPool>,
    Json(todo): Json<NewTodoRequest>,
) -> Result<Json<TodoView>, ApiError> {
    let todo = todo_service::create_todo(pool, todo).await;

    match todo {
        Ok(todo) => Ok(Json(TodoView::from(todo))),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}

pub async fn find_todos(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<PaginationParams>,
) -> Result<Json<Vec<TodoView>>, ApiError> {
    let (page, limit) = (query.page.unwrap_or(1), query.limit.unwrap_or(10));

    let todos = todo_service::find_todos(pool, page, limit).await;

    match todos {
        Ok(todos) => Ok(Json(
            todos.into_iter().map(|todo| TodoView::from(todo)).collect(),
        )),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}

pub async fn find_todo_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<TodoView>, ApiError> {
    let todo = todo_service::find_todo_by_id(pool, id).await;

    match todo {
        Ok(todo) => Ok(Json(TodoView::from(todo))),
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
    Json(todo): Json<EditTodoRequest>,
) -> Result<Json<TodoView>, ApiError> {
    let todo = todo_service::edit_todo_by_id(pool, id, todo).await;

    match todo {
        Ok(todo) => Ok(Json(TodoView::from(todo))),
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
    let result = todo_service::delete_todo_by_id(pool, id).await;

    match result {
        Ok(_) => Ok(Json(())),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}
