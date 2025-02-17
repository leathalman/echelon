use crate::api::router::AppState;
use crate::storage::postgres::MessageRole;
use crate::storage::postgres::{Conversation, Message};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

/// GET /api/conversation/
/// Authorized Endpoint -> JWT Required
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
            "message": "Something bad happened while fetching notes..."
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let conversations = query_result.unwrap();

    let json_response = serde_json::json!({
        "conversations": conversations
    });

    Ok(Json(json_response))
}

/// GET /api/conversation/{conversation_id}/messages
/// Authorized Endpoint -> JWT Required
pub async fn conversation_list_messages(
    Path(conversation_id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
            Message,
            r#"
            SELECT id, conversation_id, content, role as "role!: MessageRole", created_at as "created_at:DateTime<Utc>"
            FROM chat.messages
            WHERE conversation_id = $1
            ORDER BY created_at ASC
            "#,
            conversation_id
        )
        .fetch_all(&state.db)
        .await;

    if query_result.is_err() {
        let error_response = json!({
            "message": "Something bad happened while fetching messages..."
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let messages = query_result.unwrap();

    let json_response = json!({
        "messages": messages
    });

    Ok(Json(json_response))

    // TODO: Only return success if UserID matches ownerID from Postgres, aka needs security
}

/// POST /api/conversation/{conversation_id}/messages
#[axum::debug_handler]
pub async fn conversation_new_message_handler(
    Path(conversation_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMessageSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("Convo ID {}", conversation_id);

    let query_result = sqlx::query_as!(
        Message,
        r#"
                WITH new_message AS (
                INSERT INTO chat.messages (conversation_id, content, role)
                VALUES ($1, $2, $3)
                RETURNING id, conversation_id, content, role as "role!: MessageRole", created_at
            ),
            update_conversation AS (
                UPDATE chat.conversations
                SET last_message_at = CURRENT_TIMESTAMP
                WHERE id = $1
            )
            SELECT * FROM new_message
            "#,
        conversation_id,
        payload.content,
        payload.role as MessageRole
    )
        .fetch_one(&state.db)
        .await;

    match query_result {
        Ok(message) => {
            let message_response = json!({
                "message_id": message.id
            });

            Ok((StatusCode::CREATED, Json(message_response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = json!({
                    "message": "Message with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": format!("{:?}", e)})),
            ))
        }
    }
}

// put whatever needs to be in the JSON body in here
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageSchema {
    pub content: String,
    pub role: MessageRole,
}
