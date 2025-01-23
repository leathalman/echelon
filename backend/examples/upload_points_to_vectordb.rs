use backend::embeddings::process_file;
use backend::vectordb;
use backend::vectordb::vector_store::VectorStoreType;
use backend::vectordb::vector_store::{VectorStore, VectorStorePoint};
use fastembed::EmbeddingModel;
use std::path::Path;

#[tokio::main]
async fn main() {
    let vectordb = vectordb::build(VectorStoreType::Qdrant).unwrap();

    vectordb
        .create_collection("test_collection", EmbeddingModel::AllMiniLML6V2)
        .await
        .unwrap();

    let path = Path::new("./data/John_V_Roach_Honors_College.md");

    let chunks: Vec<VectorStorePoint> = process_file(path)
        .unwrap()
        .into_iter()
        .map(|chunk| chunk.into())
        .collect();

    vectordb
        .add_vectors("test_collection", chunks)
        .await
        .unwrap();
}
