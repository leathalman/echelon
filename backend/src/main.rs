mod llm;
mod vectordb;

use crate::llm::inference::Inference;
use crate::llm::inference::InferenceRequest;
use crate::llm::model::Model;
use crate::vectordb::VectorDBType;
use fastembed::TextEmbedding;

#[tokio::main]
async fn main() {
    // let model = TextEmbedding::try_new(Default::default()).unwrap();
    //
    // let documents = vec!["What are all of the classes that Harrison has taken?"];
    //
    // let embeddings = model.embed(documents, None).unwrap();
    //
    // println!("Embeddings length: {}", embeddings.len());
    // println!("Embeddings dimension: {}", embeddings[0].len());
    //
    // let client = match vectordb::build(VectorDBType::Qdrant) {
    //     Ok(client) => client,
    //     Err(err) => {
    //         println!("{}", err);
    //         std::process::exit(1)
    //     }
    // };

    // client
    //     .add_vectors(embeddings, vec!["string"], "dim", "some_file")
    //     .await
    //     .unwrap();

    // match client.create_collection("test").await {
    //     Ok(_) => println!("Collection Created Successfully"),
    //     Err(err) => {
    //         println!("{}", err);
    //         std::process::exit(1)
    //     }
    // }

    // client.query(embeddings[0].clone(), "dim").await.unwrap();
}
