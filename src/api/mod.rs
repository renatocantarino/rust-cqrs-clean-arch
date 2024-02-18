use axum::{response::IntoResponse, Json};

pub mod router;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine, thanks!";

    let response = serde_json::json!({
        "status": "Sucess",
        "message": MESSAGE
    });

    Json(response)
}
