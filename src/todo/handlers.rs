use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::common::pagination::PaginatedView;
use crate::{
    common::{errors::ApiError, middlewares::AuthState, pagination::PaginationParams},
    todo::{
        service as todo_service,
        views::{EditTodoRequest, NewTodoRequest, TodoView},
    },
};

/// Create todo
#[utoipa::path(
    post,
    operation_id = "create_todo",
    path = "/api/todos",
    tag = "todo",
    request_body = NewTodoRequest,
    responses(
        (status = 200, description = "Todo created", body = TodoView),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 500, description = "Todo creation failed", body = ApiError)
    ),
    security(("api_key" = []))
)]
pub async fn create_todo(
    State(state): State<AuthState>,
    Json(todo): Json<NewTodoRequest>,
) -> Result<Json<TodoView>, ApiError> {
    let todo = todo_service::create_todo(state.pool, todo).await;

    match todo {
        Ok(todo) => Ok(Json(TodoView::from(todo))),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}

/// Find todos
#[utoipa::path(
    get,
    operation_id = "find_todos",
    path = "/api/todos",
    tag = "todo",
    params(PaginationParams),
    responses(
        (status = 200, description = "Todos found", body = PaginatedTodoView),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 500, description = "Todos not found", body = ApiError)
    ),
    security(("api_key" = []))
)]
pub async fn find_todos(
    State(state): State<AuthState>,
    Query(query): Query<PaginationParams>,
) -> Result<Json<PaginatedView<TodoView>>, ApiError> {
    let (page, size) = (query.page.unwrap_or(1), query.size.unwrap_or(10));

    let todos = todo_service::find_todos(state.pool, page, size).await;

    match todos {
        Ok(todos) => Ok(Json(PaginatedView {
            page,
            size,
            items: todos.into_iter().map(TodoView::from).collect(),
        })),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}

/// Find todo by id
#[utoipa::path(
    get,
    operation_id = "find_todo_by_id",
    path = "/api/todos/{id}",
    tag = "todo",
    params(
        ("id" = i32, description = "Todo id")
    ),
    responses(
        (status = 200, description = "Todo found", body = TodoView),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 404, description = "Todo not found", body = ApiError),
        (status = 500, description = "Todo not found", body = ApiError)
    ),
    security(("api_key" = []))
)]
pub async fn find_todo_by_id(
    State(state): State<AuthState>,
    Path(id): Path<i32>,
) -> Result<Json<TodoView>, ApiError> {
    let todo = todo_service::find_todo_by_id(state.pool, id).await;

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

/// Edit todo by id
#[utoipa::path(
    put,
    operation_id = "edit_todo_by_id",
    path = "/api/todos/{id}",
    tag = "todo",
    params(
        ("id" = i32, description = "Todo id")
    ),
    request_body = EditTodoRequest,
    responses(
        (status = 200, description = "Todo edited", body = TodoView),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 404, description = "Todo not found", body = ApiError),
        (status = 500, description = "Todo not found", body = ApiError)
    ),
    security(("api_key" = []))
)]
pub async fn edit_todo_by_id(
    State(state): State<AuthState>,
    Path(id): Path<i32>,
    Json(todo): Json<EditTodoRequest>,
) -> Result<Json<TodoView>, ApiError> {
    let todo = todo_service::edit_todo_by_id(state.pool, id, todo).await;

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

/// Delete todo by id
#[utoipa::path(
    delete,
    operation_id = "delete_todo_by_id",
    path = "/api/todos/{id}",
    tag = "todo",
    params(
        ("id" = i32, description = "Todo id")
    ),
    responses(
        (status = 200, description = "Todo deleted"),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 500, description = "Todo not found", body = ApiError)
    ),
    security(("api_key" = []))
)]
pub async fn delete_todo_by_id(
    State(state): State<AuthState>,
    Path(id): Path<i32>,
) -> Result<(), ApiError> {
    let result = todo_service::delete_todo_by_id(state.pool, id).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::new_internal(e.to_string())),
    }
}
