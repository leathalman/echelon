use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

use crate::api::health::health_checker_handler;
use crate::api::conversations::conversation_list_handler;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/conversations", get(conversation_list_handler))
        .with_state(app_state)
}
