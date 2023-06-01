use axum::{extract::State, Json};

use crate::{
    auth::{models::JWT_SECRET, utils, views::LoginRequest, views::TokenView},
    common::{errors::ApiError, middlewares::AuthState},
    user::service as user_service,
};

static ACCESS_EXPIRY: usize = 60 * 60 * 100000000;
static REFRESH_EXPIRY: usize = 24 * 60 * 60 * 100000000;

pub async fn get_tokens(
    State(AuthState {
        pool,
        jwt_secret: _,
    }): State<AuthState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<TokenView>, ApiError> {
    let LoginRequest { email, password } = request;

    let user = user_service::login(pool, &email, &password).await;

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
