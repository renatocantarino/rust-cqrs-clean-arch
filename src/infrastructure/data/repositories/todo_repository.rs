use crate::{domain::models::todo::Todo, infrastructure::data::db_context::surreal_context::DB};
use surrealdb::{error::Db::Thrown, Error};

pub struct TodoRepository {
    table: String,
}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {
            table: String::from("todo"),
        }
    }

    pub async fn create_todo(&self, data: Todo) -> Result<Vec<Todo>, Error> {
        let row = DB.create(&self.table).content(data).await?;
        Ok(row)
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let records = DB.select(&self.table).await?;
        Ok(records)
    }
}
