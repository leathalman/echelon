use crate::storage::qdrant::QdrantAdapter;
use async_trait::async_trait;
use fastembed::{Embedding, EmbeddingModel};
use thiserror::Error;

pub enum VectorStorageBackend {
    Qdrant,
}

#[async_trait]
pub trait VectorStorage {
    async fn create_collection(
        &self,
        collection_name: &str,
        embedding_model: EmbeddingModel,
    ) -> Result<VectorStorageResponse, VectorStorageError>;
    async fn query(
        &self,
        collection_name: &str,
        vector: Embedding,
    ) -> Result<VectorStorageQueryResponse, VectorStorageError>;
    async fn add_vectors(
        &self,
        collection_name: &str,
        points: Vec<VectorDataPoint>,
    ) -> Result<VectorStorageResponse, VectorStorageError>;
}

pub fn build(backend: VectorStorageBackend) -> Result<impl VectorStorage, VectorStorageError> {
    match backend {
        VectorStorageBackend::Qdrant => QdrantAdapter::new(),
    }
}

pub struct VectorStorageResponse {
    pub time: f64,
}

#[derive(Debug)]
pub struct VectorStorageQueryResponse {
    pub time: f64,
    pub points: Vec<VectorDataPoint>,
}

#[derive(Debug)]
pub struct VectorDataPoint {
    pub uuid: String,
    // maps to 'name' key in DB
    pub name: String,
    // maps to 'content' key in DB
    pub content: String,
    pub embedding: Option<Embedding>,
    pub score: Option<f32>,
}

#[derive(Debug, Error)]
pub enum VectorStorageError {
    #[error("VectorStore error occurred: {0}")]
    Message(String),
}
