use axum::{
    routing::{get, post},
    Router,
};

use crate::application::{
    commands::create_todo_command::create_todo_command, queries::get_all_todos_query,
};

use super::health_checker_handler;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/hc", get(health_checker_handler))
        .route("/api/todos", post(create_todo_command))
        .route("/api/todos", get(get_all_todos_query))
}
