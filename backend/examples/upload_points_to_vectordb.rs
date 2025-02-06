use backend::processing::chunk::chunk;
use backend::processing::document::Document;
use backend::processing::embedding::embed_batch;
use backend::processing::utils::compile_vectors;
use backend::storage::vector;
use backend::storage::vector::{VectorStorage, VectorStorageBackend};
use fastembed::EmbeddingModel;
use std::error::Error;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let vectordb = vector::build(VectorStorageBackend::Qdrant)?;

    vectordb
        .create_collection("some_collection", EmbeddingModel::AllMiniLML6V2)
        .await?;

    let document = Document::new(Path::new("./data/John_V_Roach_Honors_College.md"))?;

    let chunks = chunk(document.content)?;

    let embeddings = embed_batch(&chunks)?;

    let vectors = compile_vectors(document.file_name, chunks, embeddings)?;

    vectordb.add_vectors("some_collection", vectors).await?;

    Ok(())
}
