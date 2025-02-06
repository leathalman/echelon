use backend::storage::postgres::{MessageRole, PostgresAdapter};
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Reads the .env file

    let db_url = env::var("DATABASE_URL")?;

    let db = PostgresAdapter::new(&db_url).await?;

    // let user = db.create_user("123456", "hleath@me.com", "hash").await?;

    // let user = db.get_user_by_email("hleath@me.com").await?;

    // let conversation = db.create_conversation(user.id, "Asteroids are cool!").await?;
    // let conversation2 = db.create_conversation(user.id, "What is Rust anyways?").await?;
    //
    // let conversations = db.get_user_conversations(1).await?;

    // let message = db
    //     .create_message(1, "I don't know about that...", MessageRole::Assistant)
    //     .await?;

    let messages = db.get_conversation_messages(1).await?;

    // println!("{:?}", user);

    println!("{:?}", messages);

    Ok(())
}
