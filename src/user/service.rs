use sqlx::PgPool;

use crate::common::password_encoder;
use crate::user::{models::User, repository as user_repository, views::NewUserRequest};

pub async fn register_user(
    pool: PgPool,
    new_user_request: NewUserRequest,
) -> Result<User, sqlx::Error> {
    let new_user_request = NewUserRequest {
        password: password_encoder::encode_password(new_user_request.password.as_str()),
        ..new_user_request
    };

    user_repository::register_user(pool, new_user_request).await
}

pub async fn find_user_by_email(pool: PgPool, email: &str) -> Result<User, sqlx::Error> {
    user_repository::find_user_by_email(pool, email).await
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