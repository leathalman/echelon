use chrono::{DateTime, Utc};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use backend::storage::postgres::{Conversation};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tracing::{error, info};

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Return error if .env does not exist
    dotenv::dotenv().ok();

    // Initialize logger
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a single connection pool for SQLx that's shared across the entire application.
    let db = match PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await {
        Ok(pool) => {
            info!("Successfully connected to database");
            pool
        }
        Err(err) => {
            error!("Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    };

    let app_state = Arc::new(AppState { db });
    let app = Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/conversations", get(convo_list_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Echelon is running :)";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

async fn convo_list_handler(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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