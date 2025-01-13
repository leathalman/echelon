use async_trait::async_trait;
use fastembed::{Embedding, EmbeddingModel};
use qdrant_client::prelude::point_id::PointIdOptions;
use qdrant_client::qdrant::ScoredPoint;
use qdrant_client::qdrant::value::Kind;
use thiserror::Error;

pub enum VectorStoreType {
    Qdrant
}

#[async_trait]
pub trait VectorStore {
    async fn create_collection(
        &self,
        collection_name: &str,
        embedding_model: EmbeddingModel,
    ) -> Result<VectorStoreResponse, VectorStoreError>;
    async fn query(
        &self,
        collection_name: &str,
        vector: Embedding,
    ) -> Result<VectorStoreQueryResponse, VectorStoreError>;
    async fn add_vectors(
        &self,
        collection_name: &str,
        vectors: Vec<Embedding>,
        payload: Vec<&str>,
    ) -> Result<VectorStoreResponse, VectorStoreError>;
}

pub struct VectorStoreResponse {
    pub time: f64,
}

#[derive(Debug)]
pub struct VectorStorePoint {
    pub id: Option<String>,
    // maps to 'content' key in DB
    pub content: Option<String>,
    // maps to 'name' key in DB
    pub name: Option<String>,
    pub score: f32,
}

impl VectorStorePoint {
    // helper function for converting from Qdrant resulting types
    pub fn from_scored_point(scored_point: ScoredPoint) -> Result<Self, VectorStoreError> {
        let id = if let Some(point_id) = scored_point.id {
            if let Some(PointIdOptions::Uuid(uuid)) = point_id.point_id_options {
                uuid
            } else {
                return Err(VectorStoreError::Message(
                    "PointStruct Conversion Error: Point ID is not a UUID.".to_string(),
                ));
            }
        } else {
            return Err(VectorStoreError::Message(
                "PointStruct Conversion Error: PointID is None.".to_string(),
            ));
        };

        let content = scored_point
            .payload
            .get("content")
            .and_then(|value| match &value.kind {
                Some(Kind::StringValue(s)) => Some(s.clone()),
                _ => None,
            })
            .ok_or_else(|| {
                VectorStoreError::Message("Missing or invalid 'content' field".to_string())
            })?;

        let name = scored_point
            .payload
            .get("name")
            .and_then(|value| match &value.kind {
                Some(Kind::StringValue(s)) => Some(s.clone()),
                _ => None,
            })
            .ok_or_else(|| {
                VectorStoreError::Message("Missing or invalid 'name' field".to_string())
            })?;

        Ok(VectorStorePoint {
            id: Some(id),
            content: Some(content),
            name: Some(name),
            score: scored_point.score,
        })
    }
}

#[derive(Debug)]
pub struct VectorStoreQueryResponse {
    pub time: f64,
    pub points: Vec<VectorStorePoint>,
}

#[derive(Debug, Error)]
pub enum VectorStoreError {
    #[error("VectorStore error occurred: {0}")]
    Message(String),
}