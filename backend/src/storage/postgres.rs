use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::time::OffsetDateTime;
use std::error::Error;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "chat.message_role", rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "chat.conversation_status", rename_all = "lowercase")]
pub enum ConversationStatus {
    Active,
    Archived,
    Starred,
    System,
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub student_id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub last_login_at: OffsetDateTime,
}

#[derive(Debug)]
pub struct Conversation {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub last_message_at: OffsetDateTime,
    pub status: ConversationStatus,
}

#[derive(Debug)]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub content: String,
    pub role: MessageRole,
    pub created_at: OffsetDateTime,
}

pub struct RelationalStorage {
    pool: PgPool,
}

impl RelationalStorage {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(RelationalStorage { pool })
    }

    pub async fn create_user(
        &self,
        student_id: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO chat.users (student_id, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, student_id, email, password_hash, created_at, last_login_at
            "#,
            student_id,
            email,
            password_hash
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM chat.users WHERE email = $1
            "#,
            email
        )
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_conversation(
        &self,
        owner_id: i32,
        title: &str,
    ) -> Result<Conversation, sqlx::Error> {
        sqlx::query_as!(
            Conversation,
            r#"
            INSERT INTO chat.conversations (owner_id, title)
            VALUES ($1, $2)
            RETURNING id, owner_id, title, last_message_at, status as "status!: ConversationStatus"
            "#,
            owner_id,
            title
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_user_conversations(
        &self,
        user_id: i32,
    ) -> Result<Vec<Conversation>, sqlx::Error> {
        sqlx::query_as!(
            Conversation,
            r#"
            SELECT id, owner_id, title, last_message_at, status as "status:_"
            FROM chat.conversations
            WHERE owner_id = $1
            ORDER BY last_message_at
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_message(
        &self,
        conversation_id: i32,
        content: &str,
        role: MessageRole,
    ) -> Result<Message, sqlx::Error> {
        sqlx::query_as!(
            Message,
            r#"
                WITH new_message AS (
                INSERT INTO chat.messages (conversation_id, content, role)
                VALUES ($1, $2, $3)
                RETURNING id, conversation_id, content, role as "role!: MessageRole", created_at
            ),
            update_conversation AS (
                UPDATE chat.conversations 
                SET last_message_at = CURRENT_TIMESTAMP
                WHERE id = $1
            )
            SELECT * FROM new_message
            "#,
            conversation_id,
            content,
            role as MessageRole
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_conversation_messages(
        &self,
        conversation_id: i32,
    ) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as!(
            Message,
            r#"
            SELECT id, conversation_id, content, role as "role!: MessageRole", created_at
            FROM chat.messages
            WHERE conversation_id = $1
            ORDER BY created_at ASC
            "#,
            conversation_id
        )
            .fetch_all(&self.pool)
            .await
    }
}
