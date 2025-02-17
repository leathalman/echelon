use chrono::{DateTime, Utc};
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::api::router::AppState;
use crate::storage::postgres::Conversation;

pub async fn conversation_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Hardcoded to user with ID=1, this will be replaced by JWT
    let user_id = 1;

    let query_result = sqlx::query_as!(
            Conversation,
            r#"
            SELECT id, owner_id, title, last_message_at as "last_message_at:DateTime<Utc>", status as "status:_"
            FROM chat.conversations
            WHERE owner_id = $1
            ORDER BY last_message_at
            "#,
            user_id
        )
        .fetch_all(&state.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching notes..."
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let convos = query_result.unwrap();

    let json_response = serde_json::json!({
        "results": convos.len(),
        "convos": convos
    });

    Ok(Json(json_response))
}