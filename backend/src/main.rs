use crate::embeddings::process_file;
use crate::llm::inference::Inference;
use crate::llm::inference::InferenceRequest;
use crate::llm::model::Model;
use std::path::Path;

mod embeddings;
mod llm;
mod vectordb;

#[tokio::main]
async fn main() {
    // let llm = llm::build(Model::Llama3_3b);
    //
    // let response = llm
    //     .generate(InferenceRequest::new("What is TCU".to_string()))
    //     .await
    //     .unwrap();
    //
    // println!("{}", response.content);

    let path = Path::new("./data/John_V_Roach_Honors_College.md");

    let result = process_file(path).unwrap();

    println!("{:?}", result);
    println!("{}", result.len());
}
