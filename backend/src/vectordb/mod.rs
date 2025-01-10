use crate::vectordb::qdrant::QdrantClient;
use async_trait::async_trait;
use fastembed::Embedding;
use thiserror::Error;

pub mod qdrant;

const QDRANT_URL: &str = "http://localhost:6334";

pub enum VectorDatabaseType {
    Qdrant,
}
#[async_trait]
#[allow(dead_code)] // temp for testing purposes, cleans up compiler output
pub trait VectorDB {
    async fn create_collection(&self, collection_name: &str) -> Result<(), VectorDBError>;
    async fn query(&self, vector: Embedding, collection_name: &str) -> Result<(), VectorDBError>;
    async fn add_vectors(
        &self,
        vectors: Vec<Embedding>,
        payload: Vec<&str>,
        collection_name: &str,
        file_name: &str,
    ) -> Result<(), VectorDBError>;
}

#[derive(Debug, Error)]
pub enum VectorDBError {
    #[error("{0}")]
    GenericError(String),
}

pub fn build(db_type: VectorDatabaseType) -> Result<Box<dyn VectorDB>, VectorDBError> {
    match db_type {
        VectorDatabaseType::Qdrant => match QdrantClient::new(QDRANT_URL) {
            Ok(qdrant_client) => Ok(Box::new(qdrant_client)),
            Err(err) => Err(err.into()),
        },
    }
}
