use crate::api::router::AppState;
use crate::storage::postgres::MessageRole;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

/// GET /api/conversation/
/// Authorized Endpoint -> JWT Required
pub async fn conversation_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Hardcoded to user with ID=1, this will be replaced by JWT
    let query_result = state.db.get_user_conversations(1).await;

    if query_result.is_err() {
        let error_response = json!({
            "message": "Something bad happened while fetching notes..."
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let conversations = query_result.unwrap();

    let json_response = json!({
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
    // TODO: hardcoded USER_ID, replace with JWT implementation
    let query_result = state.db.get_conversation_messages(conversation_id).await;

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
pub async fn conversation_new_message_handler(
    Path(conversation_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMessageSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = state.db.create_message(conversation_id, payload.content, payload.role).await;

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

// TODO: how are we going to trigger LLAMA?? Do I need to put it in AppState, or something else?

// put whatever needs to be in the JSON body in here
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageSchema {
    pub content: String,
    pub role: MessageRole,
}
