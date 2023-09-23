use std::fmt::{Display, Formatter};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Serialize, Error, Debug, ToSchema)]
pub struct ApiError {
    status_code: u16,
    errors: Vec<String>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Err {} ", &self.status_code))
    }
}

impl ApiError {
    pub fn new(status_code: u16, err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code,
            errors,
        }
    }

    pub fn new_internal(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            errors,
        }
    }
    pub fn new_bad_request(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            errors,
        }
    }

    pub fn new_unauthorized(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
            errors,
        }
    }

    pub fn new_not_found(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            errors,
        }
    }

    pub fn new_conflict(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::CONFLICT.as_u16(),
            errors,
        }
    }

    pub fn append_error(&mut self, err: String) {
        self.errors.push(err);
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status_code).unwrap(), Json(self)).into_response()
    }
}
