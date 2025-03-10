use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use backend::api::router::create_router;
use backend::app_state::AppState;
use backend::config::Config;
use backend::llm::inference;
use backend::llm::model::Model::Llama3_11b;
use backend::storage::postgres::RelationalStorage;
use backend::storage::qdrant::QdrantAdapter;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let listener = tokio::net::TcpListener::bind(&config.deployment_url).await.unwrap();

    let app = create_router(Arc::new(AppState {
        relational_storage: RelationalStorage::new(&config.postgres_url)
            .await
            .unwrap(),
        vector_storage: QdrantAdapter::new(&config.qdrant_url).unwrap(),
        llm: inference::build(Llama3_11b),
        config,
    }))
        .layer(cors);

    info!("Axum is up.");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve Axum app");
}
