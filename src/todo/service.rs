use sqlx::PgPool;

use crate::todo::{
    models::Todo,
    repository as todo_repository,
    views::{EditTodoRequest, NewTodoRequest},
};

#[derive(Clone)]
pub struct TodoService {
    pool: PgPool,
}

impl TodoService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_todo(&self, new_todo: NewTodoRequest) -> Result<Todo, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let new_todo = todo_repository::create_todo(&mut tx, new_todo).await?;
        tx.commit().await?;

        Ok(new_todo)
    }

    pub async fn find_todos(&self, page: i32, size: i32) -> Result<Vec<Todo>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let todos = todo_repository::find_todos(&mut tx, page, size).await?;
        tx.commit().await?;

        Ok(todos)
    }

    pub async fn find_todo_by_id(&self, id: i32) -> Result<Todo, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let todo = todo_repository::find_todo_by_id(&mut tx, id).await?;
        tx.commit().await?;

        Ok(todo)
    }

    pub async fn edit_todo_by_id(
        &self,
        id: i32,
        edited_todo: EditTodoRequest,
    ) -> Result<Todo, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let edited_todo = todo_repository::edit_todo_by_id(&mut tx, id, edited_todo).await?;
        tx.commit().await?;

        Ok(edited_todo)
    }

    pub async fn delete_todo_by_id(&self, id: i32) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        todo_repository::delete_todo_by_id(&mut tx, id).await?;
        tx.commit().await?;

        Ok(())
    }
}
