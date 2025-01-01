use axum::{
    extract::{Query, State},
    Extension, Json,
};
use utoipa::IntoParams;

use crate::{
    common::{errors::ApiError, middlewares::AuthState},
    user::views::{NewUserRequest, UserView},
};

use super::service::UserService;

/// Register user
#[utoipa::path(
    post,
    operation_id = "register_user",
    path = "/api/users",
    tag = "user",
    request_body = NewUserRequest,
    responses(
        (status = 200, description = "User created", body = UserView),
        (status = 400, description = "User creation failed", body = ApiError),
        (status = 409, description = "User already exists", body = ApiError),
        (status = 500, description = "User creation failed", body = ApiError)
    )
)]
pub async fn register_user(
    State(_state): State<AuthState>,
    Extension(user_service): Extension<UserService>,
    Json(request): Json<NewUserRequest>,
) -> Result<Json<UserView>, ApiError> {
    let user = user_service.register_user(request).await;

    match user {
        Ok(user) => Ok(Json(UserView::from(user))),
        Err(e) => match &e {
            sqlx::Error::Database(db_err) => {
                if db_err.constraint().is_some() {
                    Err(ApiError::new_conflict(
                        "User with this email already exists".to_string(),
                    ))
                } else {
                    Err(ApiError::new_internal(e.to_string()))
                }
            }
            _ => Err(ApiError::new_internal(e.to_string())),
        },
    }
}

#[derive(serde::Deserialize, IntoParams)]
pub struct FindUserQuery {
    pub email: Option<String>,
}

/// Find user by email
#[utoipa::path(
    get,
    operation_id = "find_user_by_email",
    path = "/api/users",
    tag = "user",
    params(FindUserQuery),
    responses(
        (status = 200, description = "User found", body = UserView),
        (status = 400, description = "Bad input", body = ApiError),
        (status = 404, description = "User not found", body = ApiError),
    ),
    security(("api_key" = []))
)]
pub async fn find_user_by_email(
    State(_state): State<AuthState>,
    Extension(user_service): Extension<UserService>,
    Query(query): Query<FindUserQuery>,
) -> Result<Json<UserView>, ApiError> {
    let email = query.email;

    let email = match email {
        Some(email) => email,
        None => {
            return Err(ApiError::new_bad_request(
                "`email` query is required".to_string(),
            ))
        }
    };

    let user = user_service.find_user_by_email(email.as_str()).await;

    match user {
        Ok(user) => Ok(Json(UserView::from(user))),
        Err(_) => Err(ApiError::new_not_found(format!(
            "User with email {} not found",
            email
        ))),
    }
}
