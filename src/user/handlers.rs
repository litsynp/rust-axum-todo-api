use axum::extract::Query;
use axum::{Extension, Json};
use sqlx::PgPool;

use crate::common::errors::ApiError;
use crate::user::{
    models::{NewUser, User},
    repository,
};

pub async fn register_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<NewUser>,
) -> Result<Json<User>, ApiError> {
    let user = repository::register_user(pool, user).await;

    match user {
        Ok(user) => Ok(Json(user)),
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

#[derive(serde::Deserialize)]
pub struct FindUserQuery {
    pub email: Option<String>,
}

pub async fn find_user_by_email(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<FindUserQuery>,
) -> Result<Json<User>, ApiError> {
    let email = query.email;

    let email = match email {
        Some(email) => email,
        None => {
            return Err(ApiError::new_bad_request(
                "`email` query is required".to_string(),
            ))
        }
    };

    let user = repository::find_user_by_email(pool, email.as_str()).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(ApiError::new_not_found(format!(
            "User with email {} not found",
            email
        ))),
    }
}
