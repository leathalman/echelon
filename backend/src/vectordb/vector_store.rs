use crate::error::VectorStoreError;
use async_trait::async_trait;
use fastembed::{Embedding, EmbeddingModel};
use qdrant_client::prelude::point_id::PointIdOptions;
use qdrant_client::qdrant::value::Kind;
use qdrant_client::qdrant::{PointStruct, ScoredPoint, Value, Vectors};
use std::collections::HashMap;

pub enum VectorStoreType {
    Qdrant,
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
        points: Vec<VectorStorePoint>,
    ) -> Result<VectorStoreResponse, VectorStoreError>;
}

pub struct VectorStoreResponse {
    pub time: f64,
}

#[derive(Debug)]
pub struct VectorStorePoint {
    pub uuid: Option<String>,
    // maps to 'content' key in DB
    pub content: Option<String>,
    pub embedding: Option<Embedding>,
    // maps to 'name' key in DB
    pub name: Option<String>,
    pub score: Option<f32>,
}

impl VectorStorePoint {
    // helper function for converting from Qdrant resulting types
    pub fn from_scored_point(scored_point: ScoredPoint) -> Result<Self, VectorStoreError> {
        let id = if let Some(point_id) = scored_point.id {
            if let Some(PointIdOptions::Uuid(uuid)) = point_id.point_id_options {
                uuid
            } else {
                return Err(VectorStoreError::Message(
                    "ScoredPoint Conversion Error: Point ID is not a UUID.".to_string(),
                ));
            }
        } else {
            return Err(VectorStoreError::Message(
                "ScoredPoint Conversion Error: PointID is None.".to_string(),
            ));
        };

        /* TODO: look into generalizing 'content' and 'name' keys - don't love that they are hardcoded strings
          Perhaps an enum is a better idea: 'VectorStorePayloadKeys'
        */
        let content = scored_point
            .payload
            .get("content")
            .and_then(|value| match &value.kind {
                Some(Kind::StringValue(s)) => Some(s.clone()),
                _ => None,
            })
            .ok_or_else(|| {
                VectorStoreError::Message(
                    "ScoredPoint Conversion Error: Missing or invalid 'content' field".to_string(),
                )
            })?;

        let name = scored_point
            .payload
            .get("name")
            .and_then(|value| match &value.kind {
                Some(Kind::StringValue(s)) => Some(s.clone()),
                _ => None,
            })
            .ok_or_else(|| {
                VectorStoreError::Message(
                    "ScoredPoint Conversion Error: Missing or invalid 'name' field".to_string(),
                )
            })?;

        Ok(VectorStorePoint {
            uuid: Some(id),
            content: Some(content),
            name: Some(name),
            score: Some(scored_point.score),
            embedding: None,
        })
    }

    // TODO: hardcoded keys for VectorStore, should generalize
    pub fn to_point_struct(self) -> Result<PointStruct, VectorStoreError> {
        let id = if let Some(val) = self.uuid {
            val
        } else {
            return Err(VectorStoreError::Message(
                "PointStruct Conversion Error: ID is none.".to_string(),
            ));
        };

        let vectors = if let Some(val) = self.embedding {
            val
        } else {
            return Err(VectorStoreError::Message(
                "PointStruct Conversion Error: Vectors are none.".to_string(),
            ));
        };

        let name = if let Some(val) = self.name {
            val
        } else {
            return Err(VectorStoreError::Message(
                "PointStruct Conversion Error: Name is none.".to_string(),
            ));
        };

        let content = if let Some(val) = self.content {
            val
        } else {
            return Err(VectorStoreError::Message(
                "PointStruct Conversion Error: Content is none.".to_string(),
            ));
        };

        let mut payload: HashMap<String, Value> = HashMap::new();
        payload.insert("content".to_string(), content.into());
        payload.insert("name".to_string(), name.into());

        Ok(PointStruct::new(id, Vectors::from(vectors), payload))
    }
}

#[derive(Debug)]
pub struct VectorStoreQueryResponse {
    pub time: f64,
    pub points: Vec<VectorStorePoint>,
}
