use backend::llm::inference;
use backend::llm::inference::{Inference, InferenceRequest};
use backend::llm::model::Model;
use backend::llm::prompt::Prompt;
use backend::vectorization::embedding::embed;
use backend::storage;
use backend::storage::postgres::{DBMessageRole, RelationalStorage};
use backend::storage::vector::{VectorStorage, VectorStorageBackend};
use dotenv::dotenv;
use std::error::Error;
use std::io::Write;
use std::{env, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let db = RelationalStorage::new(&db_url).await?;

    let user = db.get_user_by_email("hleath@me.com").await?.unwrap();
    let conversation = db.create_conversation(user.id, "Demo Convo").await?;

    let vectordb = storage::vector::build(VectorStorageBackend::Qdrant)?;
    let llm = inference::build(Model::Llama3_11b);

    loop {
        print!("Enter your query (or 'quit' to exit): ");
        io::stdout().flush()?; // Ensure the prompt is displayed before reading input

        let mut query = String::new();
        io::stdin().read_line(&mut query)?;

        // Trim whitespace and handle empty input
        let query = query.trim();

        // save user query in Postgres
        db.create_message(conversation.id, query, DBMessageRole::User)
            .await?;

        if query.is_empty() {
            println!("Query cannot be empty. Please try again.");
            continue;
        }

        if query.to_lowercase() == "quit" {
            println!("Exiting program...");
            break;
        }

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

        // save assistant's response in Postgres
        db.create_message(conversation.id, &response.content, DBMessageRole::Assistant)
            .await?;
    }

    Ok(())
}
