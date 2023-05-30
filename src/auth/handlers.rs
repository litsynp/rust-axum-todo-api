use axum::{Extension, Json};
use sqlx::PgPool;

use crate::{
    auth::views::TokenView,
    auth::{utils, views::LoginRequest},
    common::errors::ApiError,
    user::service as user_service,
};

static ACCESS_EXPIRY: usize = 60 * 60 * 1000;
static REFRESH_EXPIRY: usize = 24 * 60 * 60 * 1000;
static JWT_SECRET: &str = "foobar";

pub async fn get_tokens(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<TokenView>, ApiError> {
    let LoginRequest { email, password } = request;

    let user = user_service::login(pool, &email, &password).await;

    match user {
        Ok(user) => {
            let access_token =
                utils::encode_token(&user, ACCESS_EXPIRY, "access".to_string(), JWT_SECRET)
                    .unwrap();
            let refresh_token =
                utils::encode_token(&user, REFRESH_EXPIRY, "refresh".to_string(), JWT_SECRET)
                    .unwrap();

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
