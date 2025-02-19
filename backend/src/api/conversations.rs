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
            "status": "error",
            "code": 500,
            "message": "Failed to fetch conversations"
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let conversations = query_result.unwrap();

    let json_response = json!({
        "conversations": conversations
    });

    Ok(Json(json_response))
}

/// POST /api/conversation
pub async fn conversation_new_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateConversationSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO: hardcoded USER_ID, replace with JWT implementation
    let query_result = state.db.create_conversation(1, payload.title).await;

    match query_result {
        Ok(conversation) => {
            let conversation_response = json!({
                "conversation_id": conversation.id
            });

            Ok((StatusCode::CREATED, Json(conversation_response)))
        }
        // other errors:     "message": "error returned from database: duplicate key value violates unique constraint \"conversations_pkey\"",
        
        Err(e) => {
            // TODO: make sure this doesn't leak sensitive info
            let error_response = json!({
            "status": "error",
            "code": 500,
            "message": e.to_string(),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
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
            "status": "error",
            "code": 500,
            "message": format!("Failed to fetch messages for conversation {}", {conversation_id})
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
                "conversation_id": message.id
            });

            Ok((StatusCode::CREATED, Json(message_response)))
        }
        Err(e) => {
            // TODO: make sure this doesn't leak sensitive info
            let error_response = json!({
            "status": "error",
            "code": 500,
            "message": e.to_string(),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateConversationSchema {
    pub title: String,
}