use crate::processing::chunk::ChunkError;
use crate::storage::vector::VectorDataPoint;
use fastembed::Embedding;
use uuid::Uuid;

// Generates a vector of VectorDataPoints for use with vectordbs
pub fn compile_vectors(
    file_name: String,
    chunks: Vec<String>,
    embeddings: Vec<Embedding>,
) -> Result<Vec<VectorDataPoint>, ChunkError> {
    let mut vector_data_points: Vec<VectorDataPoint> = vec![];

    for (idx, chunk) in chunks.iter().enumerate() {
        let name = format!("{}_{}", file_name, idx);

        let embedding = match embeddings.get(idx) {
            Some(e) => e.clone(),
            None => return Err(ChunkError::Message("yikes".to_string())),
        };

        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes());

        vector_data_points.push(VectorDataPoint {
            uuid: uuid.to_string(),
            name,
            content: chunk.clone(),
            embedding: Some(embedding),
            score: None,
        })
    }

    Ok(vector_data_points)
}
