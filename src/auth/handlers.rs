use axum::{extract::State, Extension, Json};

use crate::{
    auth::{
        models::JWT_SECRET,
        utils,
        views::{LoginRequest, TokenView},
    },
    common::{errors::ApiError, middlewares::AuthState},
    user::service::UserService,
};

static ACCESS_EXPIRY: usize = 60 * 60 * 100000000;
static REFRESH_EXPIRY: usize = 24 * 60 * 60 * 100000000;

/// Get tokens for login
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
)]
pub async fn get_tokens(
    State(_store): State<AuthState>,
    Extension(user_service): Extension<UserService>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<TokenView>, ApiError> {
    let LoginRequest { email, password } = request;

    let user = user_service.login(&email, &password).await;

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
