use sqlx::PgPool;

use crate::common::password_encoder;
use crate::user::views::UserView;
use crate::user::{repository as user_repository, views::NewUserRequest};

pub async fn register_user(
    pool: PgPool,
    new_user_request: NewUserRequest,
) -> Result<UserView, sqlx::Error> {
    let new_user_request = NewUserRequest {
        password: password_encoder::encode_password(new_user_request.password.as_str()),
        ..new_user_request
    };

    let user = user_repository::register_user(pool, new_user_request).await?;

    Ok(UserView::from(user))
}

pub async fn find_user_by_email(pool: PgPool, email: &str) -> Result<UserView, sqlx::Error> {
    let user = user_repository::find_user_by_email(pool, email).await?;

    Ok(UserView::from(user))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_register_user(pool: PgPool) {
        let email = "john.doe@example.com".to_string();
        let password = "password".to_string();
        let nickname = "John Doe".to_string();

        let new_user_request = NewUserRequest {
            email,
            password,
            nickname,
        };

        let user = register_user(pool, new_user_request).await.unwrap();

        assert_eq!(user.email, "john.doe@example.com");
        assert_eq!(user.nickname, "John Doe");
    }
}
