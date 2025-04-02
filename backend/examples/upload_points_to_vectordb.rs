use backend::vectorization::chunk::{chunk, chunk_by_words};
use backend::vectorization::document::Document;
use backend::vectorization::embedding::embed_batch;
use backend::vectorization::utils::compile_vectors;
use backend::storage::vector::{VectorStorage};
use fastembed::EmbeddingModel;
use std::error::Error;
use std::path::Path;
use tracing::error;
use backend::config::{Config, Environment};
use backend::storage::qdrant::QdrantAdapter;

// DONT UPLOAD QUICK FACTS

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

    // vector_storage
    //     .create_collection(db_collection, EmbeddingModel::BGELargeENV15)
    //     .await?;

    // bge large

    let document = Document::new(Path::new("/Users/harrison/Developer/echelon/backend/data/Common_Dataset.md"))?;

    let max_words = 1000;
    let overlap = max_words / 10; // 10% overlap

    let chunks = chunk_by_words(document.content, max_words, overlap)?;

    // print!("{:?}", chunks.len());
    // print!("{:?}", chunks);

    let embeddings = embed_batch(&chunks)?;

    let vectors = compile_vectors(document.file_name, chunks, embeddings)?;

    vector_storage.add_vectors(db_collection, vectors).await?;

    Ok(())
}
