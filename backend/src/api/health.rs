use axum::Json;
use axum::response::IntoResponse;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Echelon is running :)";

    let json_response = serde_json::json!({
        "message": MESSAGE
    });

    Json(json_response)
}