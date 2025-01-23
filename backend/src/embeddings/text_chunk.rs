use crate::vectordb::vector_store::VectorStorePoint;
use fastembed::Embedding;
use uuid::Uuid;

#[derive(Debug)]
pub struct TextChunk {
    pub uuid: Uuid,
    pub content: String,
    pub embedding: Embedding,
    pub name: String,
}

impl From<TextChunk> for VectorStorePoint {
    fn from(chunk: TextChunk) -> Self {
        VectorStorePoint {
            uuid: Some(chunk.uuid.to_string()),
            content: Some(chunk.content),
            embedding: Some(chunk.embedding),
            name: Some(chunk.name),
            score: None,
        }
    }
}
