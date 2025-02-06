use std::env;
use dotenv::dotenv;
use std::error::Error;
use sqlx::Postgres;
use backend::storage;
use backend::storage::postgres::PostgresAdapter;

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
    let conversations = db.get_user_conversations(1).await?;


    // println!("{:?}", user);
    
    println!("{:?}", conversations);


    Ok(())
}