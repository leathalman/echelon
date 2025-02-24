use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use backend::api::router::create_router;
use backend::app_state::AppState;
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

    let deployment_url = std::env::var("DEPLOYMENT_URL").expect("DEPLOYMENT_URL must be set");
    let relational_database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        relational_storage: RelationalStorage::new(&relational_database_url)
            .await
            .unwrap(),
        vector_storage: QdrantAdapter::new(&qdrant_url).unwrap(),
        llm: inference::build(Llama3_11b),
    }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(deployment_url).await.unwrap();

    info!("Axum is up :)");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve Axum app");
}
