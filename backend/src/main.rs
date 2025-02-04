use dotenv::dotenv;
use std::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Reads the .env file

    let connection_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not found in .env");
    let pool = PgPool::connect(&connection_url).await?;

    let uuid = add_user(&pool).await?;

    println!("Successfully added UUID: {}", uuid);

    Ok(())
}

async fn add_user(pool: &PgPool) -> anyhow::Result<Uuid> {  // Changed return type to Uuid
    let rec = sqlx::query!(
        r#"
INSERT INTO users
(id, email, username, password_hash, full_name, status)
VALUES
    ('11111111-1111-1111-1111-111111111111', 'alice@email.com', 'alice', 'hash1', 'Alice Johnson', 'active')
RETURNING id
        "#
    )
        .fetch_one(pool)
        .await?;

    Ok(rec.id)
}