use crate::user::models::{NewUser, User};
use sqlx::PgPool;

pub async fn register_user(pool: PgPool, new_user: NewUser) -> Result<User, sqlx::Error> {
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
    .bind(new_user.email)
    .bind(new_user.password)
    .bind(new_user.nickname)
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
