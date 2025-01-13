use crate::vectordb::{VectorDB, VectorDBError};
use async_trait::async_trait;
use fastembed::Embedding;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, QueryPointsBuilder, ScalarQuantizationBuilder,
    UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::{Qdrant, QdrantError};
use uuid::Uuid;

pub struct QdrantClient {
    client: Qdrant,
    metric: Distance,
    vector_size: u64,
}

impl From<QdrantError> for VectorDBError {
    fn from(qdrant_error: QdrantError) -> Self {
        match qdrant_error {
            QdrantError::ResponseError { status } => {
                VectorDBError::GenericError(status.to_string())
            }
            QdrantError::ConversionError(str) => VectorDBError::GenericError(str),
            QdrantError::InvalidUri(str) => VectorDBError::GenericError(str.to_string()),
            QdrantError::NoSnapshotFound(str) => VectorDBError::GenericError(str),
            QdrantError::Io(err) => VectorDBError::GenericError(err.to_string()),
            QdrantError::Reqwest(err) => VectorDBError::GenericError(err.to_string()),
            QdrantError::JsonToPayload(json) => VectorDBError::GenericError(json.to_string()),
        }
    }
}

impl QdrantClient {
    // TODO: vector should change based on embedding model
    pub fn new(url: &str) -> Result<Self, QdrantError> {
        match Qdrant::from_url(url).build() {
            Ok(client) => Ok(Self {
                client,
                metric: Distance::Cosine,
                vector_size: 384,
            }),
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl VectorDB for QdrantClient {
    async fn create_collection(&self, name: &str) -> Result<(), VectorDBError> {
        match self
            .client
            .create_collection(
                CreateCollectionBuilder::new(name.to_string())
                    .vectors_config(VectorParamsBuilder::new(self.vector_size, self.metric))
                    .quantization_config(ScalarQuantizationBuilder::default()),
            )
            .await
        {
            Ok(response) => {
                if response.result {
                    Ok(())
                } else {
                    Err(VectorDBError::GenericError(name.to_string()))
                }
            }

            Err(err) => Err(VectorDBError::GenericError(err.to_string())),
        }
    }

    async fn query(&self, vector: Embedding, collection_name: &str) -> Result<(), VectorDBError> {
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
                dbg!(query_response);
                Ok(())
            }
            Err(err) => Err(VectorDBError::GenericError(err.to_string())),
        }
    }

    async fn add_vectors(
        &self,
        vectors: Vec<Embedding>,
        payload: Vec<&str>,
        collection_name: &str,
        file_name: &str,
    ) -> Result<(), VectorDBError> {
        // SRP violation -> should be in a different place
        let mut points: Vec<PointStruct> = vec![];
        for (index, vector) in vectors.iter().enumerate() {
            let file_name_with_chunk = format!("{}_chunk_{}", file_name, index);
            let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, file_name_with_chunk.as_bytes());

            // get payload
            let chunk = payload[index];

            // construct point struct and push
            points.push(PointStruct::new(
                uuid.to_string(),
                vector.to_vec(),
                [
                    ("content", chunk.into()),
                    ("name", file_name_with_chunk.into()),
                ],
            ));
        }

        match self
            .client
            .upsert_points(UpsertPointsBuilder::new(collection_name, points).wait(true))
            .await
        {
            Ok(response) => {
                if response.result.is_some() {
                    Ok(())
                } else {
                    Err(VectorDBError::GenericError(
                        "Error uploading vectors to Qdrant".to_string(),
                    ))
                }
            }
            Err(err) => Err(VectorDBError::GenericError(err.to_string())),
        }
    }
}
