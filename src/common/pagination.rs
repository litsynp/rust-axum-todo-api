use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::todo::views::TodoView;

#[derive(Deserialize, IntoParams)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Serialize, ToSchema)]
#[aliases(
    PaginatedTodoView = PaginatedView<TodoView>,
)]
pub struct PaginatedView<T> {
    pub page: i32,
    pub size: i32,
    pub items: Vec<T>,
}
