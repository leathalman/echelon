use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("Embedding error occurred: {0}")]
    Message(String),
}

#[derive(Debug, Error)]
pub enum InferenceError {
    #[error("Inference error occurred: {0}")]
    Message(String),
}

#[derive(Debug, Error)]
pub enum VectorStoreError {
    #[error("VectorStore error occurred: {0}")]
    Message(String),
}
