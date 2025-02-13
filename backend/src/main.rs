use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello world! What's good."
}

