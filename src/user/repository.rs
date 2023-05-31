use sqlx::PgPool;

use crate::user::{models::User, views::NewUserRequest};

pub async fn register_user(
    pool: PgPool,
    new_user_request: NewUserRequest,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "
            INSERT INTO
                users
                (
                    email,
                    password,
                    nickname
                )
            VALUES (
                $1,
                $2,
                $3
            )
            RETURNING
                *
            ",
    )
    .bind(new_user_request.email)
    .bind(new_user_request.password)
    .bind(new_user_request.nickname)
    .fetch_one(&pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email_and_password(
    pool: PgPool,
    email: &str,
    password: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "
          SELECT
              *
          FROM
              users
          WHERE
              email = $1 AND
              password = $2 AND
              deleted_at IS NULL
          ",
    )
    .bind(email)
    .bind(password)
    .fetch_one(&pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(pool: PgPool, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "
            SELECT
                *
            FROM
                users
            WHERE
                email = $1 AND
                deleted_at IS NULL
            ",
    )
    .bind(email)
    .fetch_one(&pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_id(pool: PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "
            SELECT
                *
            FROM
                users
            WHERE
                id = $1 AND
                deleted_at IS NULL
            ",
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(user)
}
