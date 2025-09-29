use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, query, query_as};

use crate::error::Error;

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    body: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    body: String,
}

impl CreateTodo {
    pub fn body(&self) -> &str {
        &self.body.as_ref()
    }
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    body: String,
    completed: bool,
}

impl UpdateTodo {
    pub fn body(&self) -> &str {
        &self.body.as_ref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

impl Todo {
    pub async fn list(db_pool: SqlitePool) -> Result<Vec<Todo>, Error> {
        query_as("select * from todos")
            .fetch_all(&db_pool)
            .await
            .map_err(Into::into)
    }

    pub async fn read(db_pool: SqlitePool, id: i64) -> Result<Todo, Error> {
        query_as("select * from todos where id = ?")
            .bind(id)
            .fetch_one(&db_pool)
            .await
            .map_err(Into::into)
    }

    pub async fn create(db_pool: SqlitePool, new_todo: CreateTodo) -> Result<Todo, Error> {
        query_as("insert into todos (body) values (?) returning *")
            .bind(new_todo.body())
            .fetch_one(&db_pool)
            .await
            .map_err(Into::into)
    }

    pub async fn update(
        db_pool: SqlitePool,
        id: i64,
        update_todo: UpdateTodo,
    ) -> Result<Todo, Error> {
        query_as("update todos set body = ?, completed = ? where id = ? returning *")
            .bind(update_todo.body())
            .bind(update_todo.completed())
            .bind(id)
            .fetch_one(&db_pool)
            .await
            .map_err(Into::into)
    }

    pub async fn delete(db_pool: SqlitePool, id: i64) -> Result<(), Error> {
        query("delete from todos where id =?")
            .bind(id)
            .execute(&db_pool)
            .await?;
        Ok(())
    }
}
