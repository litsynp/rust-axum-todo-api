use crate::todo::models::{EditTodo, NewTodo, Todo};
use sqlx::PgPool;

pub async fn create_todo(pool: PgPool, new_todo: NewTodo) -> Result<Todo, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
        "
            INSERT INTO
                todos
                (
                    title,
                    description
                )
            VALUES (
                $1,
                $2
            )
            RETURNING
                *
            ",
    )
    .bind(new_todo.title)
    .bind(new_todo.description)
    .fetch_one(&pool)
    .await?;

    Ok(todo)
}

pub async fn find_todos(pool: PgPool, page: i32, limit: i32) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>(
        "
            SELECT
                *
            FROM
                todos
            ORDER BY
                created_at DESC
            OFFSET
                $1
            LIMIT
                $2
            ",
    )
    .bind((page - 1) * limit)
    .bind(20)
    .fetch_all(&pool)
    .await?;

    Ok(todos)
}

pub async fn find_todo_by_id(pool: PgPool, id: i32) -> Result<Todo, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
        "
            SELECT
                *
            FROM
                todos
            WHERE
                id = $1
            ",
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(todo)
}

pub(crate) async fn edit_todo(
    pool: PgPool,
    id: i32,
    edited_todo: EditTodo,
) -> Result<Todo, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
        "
            UPDATE
                todos
            SET
                title = $1,
                description = $2,
                completed = $3,
                updated_at = NOW()
            WHERE
                id = $4
            RETURNING
                *
            ",
    )
    .bind(edited_todo.title)
    .bind(edited_todo.description)
    .bind(edited_todo.completed)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(todo)
}

pub async fn delete_todo_by_id(pool: PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
            DELETE FROM
                todos
            WHERE
                id = $1
            ",
    )
    .bind(id)
    .execute(&pool)
    .await?;

    Ok(())
}
