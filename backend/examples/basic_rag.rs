use backend::storage::vector::VectorStorage;
use std::error::Error;
use std::time::Instant;
use backend::llm::inference;
use backend::processing::embedding::*;
use backend::prompt::Prompt;
use backend::storage;
use backend::storage::backends::VectorStorageBackend;
use backend::llm::inference::*;
use backend::llm::model::Model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // RAG implementation

    let query = "What are the different distinctions in the honors college?";

    let embedded_query = embed(query.to_string())?;

    let vectordb = storage::vector::build(VectorStorageBackend::Qdrant)?;

    let search_result = vectordb
        .query("test_collection", embedded_query)
        .await?;

    let mut context = String::new();

    search_result.points.into_iter().take(5).for_each(|point| {
        context.push_str(&point.content)
    });

    let prompt = Prompt::new().question(query.to_string()).context(context);

    let start_time = Instant::now();

    let llm = inference::build(Model::Llama3_11b);

    println!("Prompt: {}", prompt.clone().to_string());

    let response = llm
        .generate(InferenceRequest::new(prompt.to_string()))
        .await
        .unwrap();

    println!("LLM Response: {}", response.content);

    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);

    println!("Elapsed time: {:.2?}", elapsed_time);

    Ok(())
}
