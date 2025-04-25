use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use backend::api::router::create_router;
use backend::app_state::AppState;
use backend::config::{Config, Environment};
use backend::llm::inference;
use backend::llm::model::Model;
use backend::storage::postgres::RelationalStorage;
use backend::storage::qdrant::QdrantAdapter;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    let config = Config::init();

    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Environment: {}", config.environment);

    let origin = match config.environment {
        Environment::Development => "http://localhost:3000",
        Environment::Production => "https://echelongpt.com"
    };

    let cors = CorsLayer::new()
        .allow_origin(origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let listener = tokio::net::TcpListener::bind(&config.deployment_url).await.unwrap();

    let qdrant_api_key = config.qdrant_api_key.clone();
    let vector_storage = match config.environment {
        Environment::Development => QdrantAdapter::new(&config.qdrant_url).unwrap(),
        Environment::Production => {
            let key = qdrant_api_key.unwrap_or_else(|e| {
                error!("No QDRANT_API_KEY set in .env, and environment is set to 'production: {}'", e);
                panic!();
            });

            QdrantAdapter::new_with_api_key(&config.qdrant_url, &key).unwrap()
        }
    };

    let app = create_router(Arc::new(AppState {
        relational_storage: RelationalStorage::new(&config.postgres_url)
            .await
            .expect("Unable to connect to Relational Storage (Postgres) from URL"),
        vector_storage,
        llm: inference::build(Model::Mistral24b, &config.ollama_url, config.ollama_port),
        config,
    }))
        .layer(cors);


    info!("Axum is up.");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve Axum app");
}
