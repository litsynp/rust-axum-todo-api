use sqlx::PgPool;

use crate::common::password_encoder;
use crate::user::models::User;
use crate::user::{repository as user_repository, views::NewUserRequest};

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn register_user(
        &self,
        new_user_request: NewUserRequest,
    ) -> Result<User, sqlx::Error> {
        let new_user_request = NewUserRequest {
            password: password_encoder::encode_password(new_user_request.password.as_str()),
            ..new_user_request
        };

        let mut tx = self.pool.begin().await?;
        let new_user = user_repository::register_user(&mut tx, new_user_request).await?;
        tx.commit().await?;

        Ok(new_user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let user = user_repository::find_user_by_email(&mut tx, email).await?;
        tx.commit().await?;

        Ok(user)
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let user = user_repository::find_user_by_id(&mut tx, id).await?;
        tx.commit().await?;

        Ok(user)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let user = user_repository::find_user_by_email(&mut tx, email).await?;
        tx.commit().await?;

        if !password_encoder::verify_password(password, user.password.as_str()) {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(user)
    }
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

        let user_service = UserService::new(pool);
        let user = user_service.register_user(new_user_request).await.unwrap();

        assert_eq!(user.email, "john.doe@example.com");
        assert_eq!(user.nickname, "John Doe");
    }
}
