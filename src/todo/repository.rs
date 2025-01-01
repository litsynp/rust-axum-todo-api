use sqlx::PgConnection;

use crate::todo::{
    models::Todo,
    views::{EditTodoRequest, NewTodoRequest},
};

pub async fn create_todo(
    conn: &mut PgConnection,
    new_todo: NewTodoRequest,
) -> Result<Todo, sqlx::Error> {
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
    .fetch_one(conn)
    .await?;

    Ok(todo)
}

pub async fn find_todos(
    conn: &mut PgConnection,
    page: i32,
    size: i32,
) -> Result<Vec<Todo>, sqlx::Error> {
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
    .bind((page - 1) * size)
    .bind(size)
    .fetch_all(conn)
    .await?;

    Ok(todos)
}

pub async fn find_todo_by_id(conn: &mut PgConnection, id: i32) -> Result<Todo, sqlx::Error> {
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
    .fetch_one(conn)
    .await?;

    Ok(todo)
}

pub async fn edit_todo_by_id(
    conn: &mut PgConnection,
    id: i32,
    edited_todo: EditTodoRequest,
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
    .fetch_one(conn)
    .await?;

    Ok(todo)
}

pub async fn delete_todo_by_id(conn: &mut PgConnection, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
            DELETE FROM
                todos
            WHERE
                id = $1
            ",
    )
    .bind(id)
    .execute(conn)
    .await?;

    Ok(())
}
