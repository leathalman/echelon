use std::sync::Arc;

use axum::{middleware, routing::get, routing::post, Router};
use crate::api::completions::{completion_new_handler, completion_new_title_handler};
use crate::api::conversations::{conversation_list_handler, conversation_list_messages, conversation_new_handler, conversation_new_message_handler};
use crate::api::health::health_checker_handler;
use crate::app_state::AppState;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use crate::api::auth::{auth_login_handler, auth_logout_handler, auth_signup_handler};
use crate::api::jwt::auth;
use crate::api::users::{user_get_handler, user_update_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    // Define a middleware stack with the auth middleware
    let auth_layer = middleware::from_fn_with_state(app_state.clone(), auth);

    // Public routes
    let public_routes = Router::new()
        .route("/health", get(health_checker_handler))
        .route("/auth/signup", post(auth_signup_handler))
        .route("/auth/login", post(auth_login_handler))
        .route("/auth/logout", get(auth_logout_handler));

    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route("/conversations", get(conversation_list_handler).post(conversation_new_handler))
        .route(
            "/conversations/:conversation_id/messages",
            get(conversation_list_messages).post(conversation_new_message_handler),
        )
        .route("/completions", post(completion_new_handler))
        .route("/completions/title", post(completion_new_title_handler))
        .route("/users", get(user_get_handler).put(user_update_handler))
        .layer(auth_layer);

    // Combine routes and add middleware
    Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        )
        .with_state(app_state)
}