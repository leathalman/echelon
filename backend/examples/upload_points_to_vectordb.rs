use backend::processing::chunk::chunk;
use backend::processing::document::Document;
use backend::processing::embedding::embed_batch;
use backend::processing::utils::compile_vectors;
use backend::storage::vector;
use backend::storage::vector::{VectorStorage, VectorStorageBackend};
use fastembed::EmbeddingModel;
use std::error::Error;
use std::path::Path;
use backend::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let config = Config::init();

    let db_collection = "course_catalog";

    let vectordb = vector::build(VectorStorageBackend::Qdrant, &config.qdrant_url).unwrap();

    vectordb
        .create_collection(db_collection, EmbeddingModel::AllMiniLML6V2)
        .await?;

    let document = Document::new(Path::new("./data/course_catalog.md"))?;

    let chunks = chunk(document.content)?;

    let embeddings = embed_batch(&chunks)?;

    let vectors = compile_vectors(document.file_name, chunks, embeddings)?;

    vectordb.add_vectors(db_collection, vectors).await?;

    Ok(())
}
