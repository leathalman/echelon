use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use backend::api::router::{create_router, AppState};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Return error if .env does not exist
    dotenv::dotenv().ok();

    // Initialize logger
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a single connection pool for SQLx that's shared across the entire application.
    let pool = match PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Successfully connected to database");
            pool
        }
        Err(err) => {
            error!("Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // TODO: Do I really need clone() here?
    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to serve Axum app");
}
