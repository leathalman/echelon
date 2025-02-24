use crate::storage::vector::{
    VectorDataPoint, VectorStorage, VectorStorageError, VectorStorageQueryResponse,
    VectorStorageResponse,
};
use async_trait::async_trait;
use fastembed::{Embedding, EmbeddingModel, TextEmbedding};
use qdrant_client::qdrant::point_id::PointIdOptions;
use qdrant_client::qdrant::value::Kind;
use qdrant_client::qdrant::Value;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, QueryPointsBuilder, ScalarQuantizationBuilder,
    ScoredPoint, UpsertPointsBuilder, VectorParamsBuilder, Vectors,
};
use qdrant_client::Qdrant;
use std::collections::HashMap;

pub struct QdrantAdapter {
    client: Qdrant,
}

impl QdrantAdapter {
    pub fn new(qdrant_url: &str) -> Result<QdrantAdapter, VectorStorageError> {
        match Qdrant::from_url(qdrant_url).build() {
            Ok(client) => Ok(Self { client }),
            Err(err) => Err(VectorStorageError::Message(format!(
                "Could not create instance of Qdrant client: {}",
                err.to_string()
            ))),
        }
    }
}

#[async_trait]
impl VectorStorage for QdrantAdapter {
    // Qdrant collections default to Cosine similarity and quantization
    async fn create_collection(
        &self,
        collection_name: &str,
        embedding_model: EmbeddingModel,
    ) -> Result<VectorStorageResponse, VectorStorageError> {
        let dim = match TextEmbedding::get_model_info(&embedding_model) {
            Ok(model_info) => model_info.dim,
            Err(err) => {
                return Err(VectorStorageError::Message(format!(
                    "Could not determine dimension of Embedding Model output: {}",
                    err.to_string()
                )))
            }
        };

        let response = self
            .client
            .create_collection(
                CreateCollectionBuilder::new(collection_name)
                    .vectors_config(VectorParamsBuilder::new(dim as u64, Distance::Cosine))
                    .quantization_config(ScalarQuantizationBuilder::default()),
            )
            .await;

        match response {
            Ok(collection_operation_response) => Ok(VectorStorageResponse {
                time: collection_operation_response.time,
            }),

            Err(e) => Err(VectorStorageError::Message(format!(
                "Unable to create Qdrant collection: {}",
                e.to_string()
            ))),
        }
    }

    async fn query(
        &self,
        collection_name: &str,
        vector: Embedding,
    ) -> Result<VectorStorageQueryResponse, VectorStorageError> {
        match self
            .client
            .query(
                QueryPointsBuilder::new(collection_name)
                    .query(vector)
                    .with_payload(true)
                    .with_vectors(true),
            )
            .await
        {
            Ok(query_response) => {
                let points: Result<Vec<VectorDataPoint>, VectorStorageError> = query_response
                    .result
                    .into_iter()
                    .map(VectorDataPoint::from_scored_point)
                    .collect();

                points.map(|points| VectorStorageQueryResponse {
                    time: query_response.time,
                    points,
                })
            }
            Err(e) => Err(VectorStorageError::Message(format!(
                "Qdrant query failed: {}",
                e.to_string()
            ))),
        }
    }

    async fn add_vectors(
        &self,
        collection_name: &str,
        points: Vec<VectorDataPoint>,
    ) -> Result<VectorStorageResponse, VectorStorageError> {
        let converted_points: Vec<PointStruct> = points
            .into_iter()
            .map(|p| p.into_point_struct())
            .collect::<Result<Vec<_>, _>>()?;

        match self
            .client
            .upsert_points(UpsertPointsBuilder::new(collection_name, converted_points).wait(true))
            .await
        {
            Ok(response) => Ok(VectorStorageResponse {
                time: response.time,
            }),
            Err(e) => Err(VectorStorageError::Message(format!(
                "Qdrant upsert vectors failed: {}",
                e.to_string()
            ))),
        }
    }
}

// type conversion for Qdrant and VectorStorage trait
impl VectorDataPoint {
    // helper function for converting from Qdrant resulting types
    pub fn from_scored_point(scored_point: ScoredPoint) -> Result<Self, VectorStorageError> {
        let uuid = if let Some(point_id) = scored_point.id {
            if let Some(PointIdOptions::Uuid(uuid)) = point_id.point_id_options {
                uuid
            } else {
                return Err(VectorStorageError::Message(
                    "ScoredPoint Conversion Error: Point ID is not a UUID.".to_string(),
                ));
            }
        } else {
            return Err(VectorStorageError::Message(
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
                VectorStorageError::Message(
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
                VectorStorageError::Message(
                    "ScoredPoint Conversion Error: Missing or invalid 'name' field".to_string(),
                )
            })?;

        Ok(VectorDataPoint {
            uuid,
            content,
            name,
            score: Some(scored_point.score),
            embedding: None,
        })
    }

    // TODO: hardcoded keys for VectorStore, should generalize
    pub fn into_point_struct(self) -> Result<PointStruct, VectorStorageError> {
        let vectors = if let Some(val) = self.embedding {
            val
        } else {
            return Err(VectorStorageError::Message(
                "PointStruct Conversion Error: Vectors/Embeddings are none.".to_string(),
            ));
        };

        let mut payload: HashMap<String, Value> = HashMap::new();
        payload.insert("content".to_string(), self.content.into());
        payload.insert("name".to_string(), self.name.into());

        Ok(PointStruct::new(self.uuid, Vectors::from(vectors), payload))
    }
}
