use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Local;
use uuid::Uuid;

use crate::domain::models::todo::Todo;

pub async fn create_todo_command(
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let local_dt = Local::now();

    body.id = Some(Uuid::new_v4().to_string());
    body.completed = Some(false);
    body.createdAt = Some(local_dt);
    body.updatedAt = Some(local_dt);

    let todo = body.to_owned();

    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}
