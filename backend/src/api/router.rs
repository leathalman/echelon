use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

use crate::api::conversations::{conversation_list_handler, conversation_list_messages};
use crate::api::health::health_checker_handler;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/conversations", get(conversation_list_handler))
        .route(
            "/api/conversations/:conversation_id/messages",
            get(conversation_list_messages),
        )
        .with_state(app_state)
}
