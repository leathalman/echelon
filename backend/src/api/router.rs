use std::sync::Arc;

use axum::{routing::get, routing::post, Router};
use crate::api::completions::completion_new_handler;
use crate::api::conversations::{conversation_list_handler, conversation_list_messages, conversation_new_handler, conversation_new_message_handler};
use crate::api::health::health_checker_handler;
use crate::app_state::AppState;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use crate::api::auth::{auth_login_handler, auth_signup_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/auth/signup", post(auth_signup_handler))
        .route("/api/auth/login", post(auth_login_handler))
        .route("/api/conversations", get(conversation_list_handler)
            .post(conversation_new_handler))
        .route(
            "/api/conversations/:conversation_id/messages",
            get(conversation_list_messages)
                .post(conversation_new_message_handler),
        )
        .route("/api/completions", post(completion_new_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        )
        .with_state(app_state)
}
