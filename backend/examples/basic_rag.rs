use backend::llm::inference::Inference;
use backend::llm::inference::InferenceRequest;
use backend::llm::model::Model;
use backend::prompt::Prompt;
use backend::vectordb::vector_store::VectorStore;
use backend::vectordb::vector_store::VectorStoreType;
use backend::{embeddings, llm, vectordb};

#[tokio::main]
async fn main() {
    let query = "What are the different distinctions in the honors college?";

    // embed query
    let query_embedded = embeddings::generate_embedding(query).unwrap();

    // sim search
    let vectordb = vectordb::build(VectorStoreType::Qdrant).unwrap();

    let search_result = vectordb
        .query("test_collection", query_embedded)
        .await
        .unwrap();

    let mut context = String::new();
    search_result.points.into_iter().take(5).for_each(|point| {
        if let Some(content) = point.content {
            context.push_str(&content)
        } else {
            println!("VectorStorePoint Error: No 'content' found in point");
        }
    });

    let prompt = Prompt::new().question(query.to_string()).context(context);

    let llm = llm::build(Model::Llama3_11b);

    println!("Prompt: {}", prompt.clone().to_string());

    let response = llm
        .generate(InferenceRequest::new(prompt.to_string()))
        .await
        .unwrap();

    println!("LLM Response: {}", response.content);
}
