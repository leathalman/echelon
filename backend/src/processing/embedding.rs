use fastembed::{Embedding, TextEmbedding};
use once_cell::sync::OnceCell;
use std::sync::Arc;
use thiserror::Error;

// TODO: Look into https://www.reddit.com/r/rust/comments/1iisfzg/lazycell_vs_lazylock_vs_oncecell_vs_oncelock_vs/
static MODEL: OnceCell<Arc<TextEmbedding>> = OnceCell::new();

pub fn get_model() -> Result<Arc<TextEmbedding>, EmbeddingError> {
    MODEL
        .get_or_try_init(|| {
            let model = TextEmbedding::try_new(Default::default()).map_err(|e| {
                EmbeddingError::Message(format!("Failed to initialize embedding model: {}", e))
            })?;
            Ok(Arc::new(model))
        })
        .cloned()
}

pub fn embed(str: String) -> Result<Embedding, EmbeddingError> {
    let model = get_model()?;

    let embeddings = model
        .embed(vec![str], Some(1))
        .map_err(|e| EmbeddingError::Message(format!("Failed to embed string: {}", e)))?;

    embeddings
        .get(0)
        .ok_or_else(|| EmbeddingError::Message("Failed to parse generated embedding".to_string()))
        .cloned()
}

pub fn embed_batch(chunks: &[String]) -> Result<Vec<Embedding>, EmbeddingError> {
    let model = get_model()?;

    // None -> use default batch size of 256
    let embeddings = model
        .embed(chunks.to_vec(), None)
        .map_err(|e| EmbeddingError::Message(format!("Failed to embed chunks: {}", e)))?;

    Ok(embeddings)
}

#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("EmbeddingError occurred: {0}")]
    Message(String),
}
