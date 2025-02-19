use std::sync::Arc;

use axum::{routing::get, Router};
use axum::routing::post;
use sqlx::{Pool, Postgres};

use crate::api::conversations::{conversation_list_handler, conversation_list_messages, conversation_new_handler, conversation_new_message_handler};
use crate::api::health::health_checker_handler;
use crate::storage::postgres::RelationalStorage;

pub struct AppState {
    pub db: RelationalStorage,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/conversations", get(conversation_list_handler)
            .post(conversation_new_handler))
        .route(
            "/api/conversations/:conversation_id/messages",
            get(conversation_list_messages)
                .post(conversation_new_message_handler),
        )
        // .route("/api/completions", post(completion_new_handler))
        .with_state(app_state)
}
