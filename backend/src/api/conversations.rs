use crate::app_state::AppState;
use crate::storage::model::{DBMessageRole, DBUser};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};
use std::time::{Instant};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageSchema {
    pub content: String,
    pub role: DBMessageRole,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateConversationSchema {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateConversationSchema {
    pub id: i32,
    pub title: String,
}

/// GET /api/conversation/
/// Authorized Endpoint -> JWT Required
/// // TODO: generalize error message return (look at auth for this)
pub async fn conversation_list_handler(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<DBUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = state
        .relational_storage
        .get_user_conversations(user.id)
        .await;

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
    Extension(user): Extension<DBUser>,
    Json(payload): Json<CreateConversationSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = state
        .relational_storage
        .create_conversation(user.id, payload.title)
        .await;

    match query_result {
        Ok(conversation) => {
            let conversation_response = json!({
                "conversation_id": conversation.id
            });

            Ok((StatusCode::CREATED, Json(conversation_response)))
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

/// GET /api/conversation/{conversation_id}/messages
/// Authorized Endpoint -> JWT Required
pub async fn conversation_list_messages(
    Path(conversation_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<DBUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let start = Instant::now();

    // fetch convo and check user id, use SQL qeury and compare
    let fetch_conversations = match state
        .relational_storage
        .get_conversation_by_id(conversation_id)
        .await {
        Ok(response) => { response }
        Err(e) => {
            error!("{}", e);
            let error_response = json!({
            "message": format!("Failed to fetch conversation {}", {conversation_id})
        });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let duration = start.elapsed();
    info!("Duration of conversation call: {}", duration.as_millis());

    if fetch_conversations.is_empty() {
        let error_response = json!({
        "message": format!("Conversation not found: {}", conversation_id)
    });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    for conversation in &fetch_conversations {
        if conversation.owner_id != user.id {
            let error_response = json!({
            "message": format!("Unauthorized access to conversation {}", conversation_id)
        });
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
    }

    let start = Instant::now();

    let query_result = state
        .relational_storage
        .get_conversation_messages(conversation_id)
        .await;

    if query_result.is_err() {
        let error_response = json!({
            "message": format!("Failed to fetch messages for conversation {}", {conversation_id})
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let duration = start.elapsed();
    info!("Duration of messages call: {}", duration.as_millis());

    let messages = query_result.unwrap();

    let json_response = json!({
        "messages": messages
    });

    Ok(Json(json_response))
}

/// POST /api/conversation/{conversation_id}/messages
pub async fn conversation_new_message_handler(
    Path(conversation_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMessageSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = state
        .relational_storage
        .create_message(conversation_id, payload.content, payload.role)
        .await;

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

pub async fn conversation_update_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateConversationSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match state
        .relational_storage
        .update_conversation(
            &payload.id,
            &payload.title,
        )
        .await
    {
        Err(e) => {
            // SQL error, failed to execute SQL
            error!("Failed on SQL fetch: {}", e.to_string());
            let error_response = json!({
                "message": "Unable to update conversation due to a server error",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
        Ok(_) => {
            let json_response = json!({
                "message": "Successfully updated conversation"
            });
            Ok((StatusCode::CREATED, Json(json_response)))
        }
    }
}