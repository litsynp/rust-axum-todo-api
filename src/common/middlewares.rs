use axum::body::Body;
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use sqlx::{Pool, Postgres};

use crate::{auth::utils::validate_token, common::errors::ApiError, user::service as user_service};

pub struct AuthState {
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}

impl Clone for AuthState {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            jwt_secret: self.jwt_secret.clone(),
        }
    }
}

pub async fn auth_middleware(
    State(AuthState { pool, jwt_secret }): State<AuthState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let header_token = match req.headers().get("Authorization") {
        Some(token) => token,
        None => return Err(ApiError::new_unauthorized("Token not found".to_string())),
    };

    let bearer_token = match header_token.to_str() {
        Ok(token) => token,
        Err(_) => return Err(ApiError::new_unauthorized("Token is not valid".to_string())),
    };

    let token = bearer_token.replace("Bearer ", "");

    let claims = match validate_token(token.as_str(), &jwt_secret) {
        Ok(claims) => claims,
        Err(err) => return Err(ApiError::new_unauthorized(err.to_string())),
    };

    let user = user_service::find_user_by_id(pool, claims.user_id).await;

    match user {
        Ok(user) => {
            req.extensions_mut().insert(user);
        }
        Err(_) => return Err(ApiError::new_unauthorized("User not found".to_string())),
    }

    Ok(next.run(req).await)
}
