use backend::processing::chunk::chunk;
use backend::processing::document::Document;
use backend::processing::embedding::embed_batch;
use backend::processing::utils::compile_vectors;
use backend::storage::vector::{VectorStorage};
use fastembed::EmbeddingModel;
use std::error::Error;
use std::path::Path;
use tracing::error;
use backend::config::{Config, Environment};
use backend::storage::qdrant::QdrantAdapter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::init();

    let vector_storage = match config.environment {
        Environment::Development => QdrantAdapter::new(&config.qdrant_url).unwrap(),
        Environment::Production => {
            let key = config.qdrant_api_key.unwrap_or_else(|e| {
                error!("No QDRANT_API_KEY set in .env, and environment is set to 'production: {}'", e);
                panic!();
            });

            QdrantAdapter::new_with_api_key(&config.qdrant_url, &key).unwrap()
        }
    };

    let db_collection = "texas_christian_university";

    vector_storage
        .create_collection(db_collection, EmbeddingModel::AllMiniLML6V2)
        .await?;

    let document = Document::new(Path::new("./data/course_catalog.md"))?;

    let chunks = chunk(document.content)?;

    let embeddings = embed_batch(&chunks)?;

    let vectors = compile_vectors(document.file_name, chunks, embeddings)?;

    vector_storage.add_vectors(db_collection, vectors).await?;

    Ok(())
}
