use backend::llm::inference;
use backend::llm::inference::{Inference, InferenceRequest};
use backend::llm::model::Model;
use backend::llm::prompt::Prompt;
use backend::processing::embedding::embed;
use backend::storage;
use backend::storage::vector::{VectorStorage, VectorStorageBackend};
use std::error::Error;
use std::io;
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let vectordb = storage::vector::build(VectorStorageBackend::Qdrant)?;
    let llm = inference::build(Model::Llama3_11b);

    loop {
        print!("Enter your query (or 'quit' to exit): ");
        io::stdout().flush()?; // Ensure the prompt is displayed before reading input

        let mut query = String::new();
        io::stdin().read_line(&mut query)?;

        // Trim whitespace and handle empty input
        let query = query.trim();

        if query.is_empty() {
            println!("Query cannot be empty. Please try again.");
            continue;
        }

        if query.to_lowercase() == "quit" {
            println!("Exiting program...");
            break;
        }

        match process_query(&vectordb, &llm, query).await {
            Ok(_) => (),
            Err(e) => println!("Error processing query: {}", e),
        }
    }

    Ok(())
}

async fn process_query(
    vectordb: &impl VectorStorage,
    llm: &impl Inference,
    query: &str,
) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let embedded_query = embed(query.to_string())?;

    let search_result = vectordb.query("test_collection", embedded_query).await?;

    let mut context = String::new();
    search_result.points.into_iter().take(5).for_each(|point| {
        if let Some(score) = point.score {
            println!("Score was: {}", score);
            if score > 0.60 {
                context.push_str(&point.content)
            }
        }
    });

    let prompt = Prompt::new(context, query.to_string());
    println!("Prompt: {}", prompt.clone().to_string());

    let response = llm
        .generate(InferenceRequest::new(prompt.to_string()))
        .await?;

    println!("LLM Response: {}", response.content);

    let elapsed_time = Instant::now().duration_since(start_time);
    println!("Elapsed time: {:.2?}", elapsed_time);
    println!();

    Ok(())
}
