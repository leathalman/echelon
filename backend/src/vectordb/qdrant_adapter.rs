use crate::error::VectorStoreError;
use crate::vectordb::vector_store::{
    VectorStore, VectorStorePoint, VectorStoreQueryResponse, VectorStoreResponse,
};
use async_trait::async_trait;
use fastembed::{Embedding, EmbeddingModel, TextEmbedding};
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, QueryPointsBuilder, ScalarQuantizationBuilder,
    UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::Qdrant;

const QDRANT_URL: &str = "http://localhost:6334";

pub struct QdrantAdapter {
    client: Qdrant,
}

impl QdrantAdapter {
    pub fn new() -> Result<impl VectorStore, VectorStoreError> {
        match Qdrant::from_url(QDRANT_URL).build() {
            Ok(client) => Ok(Self { client }),
            Err(err) => Err(VectorStoreError::Message(format!(
                "Could not create instance of Qdrant client: {}",
                err.to_string()
            ))),
        }
    }
}

#[async_trait]
impl VectorStore for QdrantAdapter {
    // Qdrant collections default to Cosine similarity and quantization
    async fn create_collection(
        &self,
        collection_name: &str,
        embedding_model: EmbeddingModel,
    ) -> Result<VectorStoreResponse, VectorStoreError> {
        let dim = match TextEmbedding::get_model_info(&embedding_model) {
            Ok(model_info) => model_info.dim,
            Err(err) => {
                return Err(VectorStoreError::Message(format!(
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
            Ok(collection_operation_response) => Ok(VectorStoreResponse {
                time: collection_operation_response.time,
            }),

            Err(e) => Err(VectorStoreError::Message(format!(
                "Unable to create Qdrant collection: {}",
                e.to_string()
            ))),
        }
    }

    async fn query(
        &self,
        collection_name: &str,
        vector: Embedding,
    ) -> Result<VectorStoreQueryResponse, VectorStoreError> {
        match self
            .client
            .query(
                QueryPointsBuilder::new(collection_name)
                    .query(vector)
                    .with_payload(true),
            )
            .await
        {
            Ok(query_response) => {
                let points: Result<Vec<VectorStorePoint>, VectorStoreError> = query_response
                    .result
                    .into_iter()
                    .map(VectorStorePoint::from_scored_point)
                    .collect();

                points.map(|points| VectorStoreQueryResponse {
                    time: query_response.time,
                    points,
                })
            }
            Err(e) => Err(VectorStoreError::Message(format!(
                "Qdrant query failed: {}",
                e.to_string()
            ))),
        }
    }

    async fn add_vectors(
        &self,
        collection_name: &str,
        points: Vec<VectorStorePoint>,
    ) -> Result<VectorStoreResponse, VectorStoreError> {
        let converted_points: Vec<PointStruct> = points
            .into_iter()
            .map(|p| p.to_point_struct())
            .collect::<Result<Vec<_>, _>>()?;

        match self
            .client
            .upsert_points(UpsertPointsBuilder::new(collection_name, converted_points).wait(true))
            .await
        {
            Ok(response) => Ok(VectorStoreResponse {
                time: response.time,
            }),
            Err(e) => Err(VectorStoreError::Message(format!(
                "Qdrant upsert vectors failed: {}",
                e.to_string()
            ))),
        }
    }
}
