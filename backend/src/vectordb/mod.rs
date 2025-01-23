pub mod qdrant_adapter;
pub mod vector_store;

use crate::error::VectorStoreError;
use crate::vectordb::qdrant_adapter::QdrantAdapter;
use crate::vectordb::vector_store::{VectorStore, VectorStoreType};

pub fn build(vector_store_type: VectorStoreType) -> Result<impl VectorStore, VectorStoreError> {
    match vector_store_type {
        VectorStoreType::Qdrant => QdrantAdapter::new(),
    }
}
