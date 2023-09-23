use axum::{extract::State, Json};

use crate::{
    auth::{models::JWT_SECRET, utils, views::LoginRequest, views::TokenView},
    common::{errors::ApiError, middlewares::AuthState},
    user::service as user_service,
};

static ACCESS_EXPIRY: usize = 60 * 60 * 100000000;
static REFRESH_EXPIRY: usize = 24 * 60 * 60 * 100000000;

/// Get tokens for login
// @formatter:off
#[utoipa::path(
    post,
    operation_id = "get_tokens",
    path = "/api/auth/tokens",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login succeeded", body = TokenView),
        (status = 400, description = "Login failed", body = ApiError)
    )
)] // @formatter:on
pub async fn get_tokens(
    State(store): State<AuthState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<TokenView>, ApiError> {
    let LoginRequest { email, password } = request;

    let user = user_service::login(store.pool, &email, &password).await;

    match user {
        Ok(user) => {
            let access_token =
                utils::encode_token(&user, ACCESS_EXPIRY, "access", JWT_SECRET).unwrap();
            let refresh_token =
                utils::encode_token(&user, REFRESH_EXPIRY, "refresh", JWT_SECRET).unwrap();

            Ok(Json(TokenView {
                user_id: user.id,
                access_token,
                refresh_token,
            }))
        }
        Err(_) => Err(ApiError::new_bad_request(
            "User not found with given credentials".to_string(),
        )),
    }
}
