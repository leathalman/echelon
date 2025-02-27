use crate::storage::model::DBConversationStatus;
use crate::storage::model::{DBConversation, DBMessage, DBMessageRole, DBUser};
use chrono::{DateTime, Utc};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::error::Error;

pub struct RelationalStorage {
    pool: PgPool,
}

impl RelationalStorage {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .connect(database_url)
            .await?;

        Ok(RelationalStorage { pool })
    }

    pub async fn create_user(
        &self,
        student_id: &str,
        email: &str,
        password_hash: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<DBUser, sqlx::Error> {
        sqlx::query_as!(
            DBUser,
            r#"
            INSERT INTO chat.users (student_id, email, password_hash, first_name, last_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, student_id, email, password_hash, first_name, last_name, created_at, last_login_at
            "#,
            student_id,
            email,
            password_hash,
            first_name,
            last_name
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<DBUser>, sqlx::Error> {
        sqlx::query_as!(
            DBUser,
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
        title: String,
    ) -> Result<DBConversation, sqlx::Error> {
        sqlx::query_as!(
            DBConversation,
            r#"
            INSERT INTO chat.conversations (owner_id, title)
            VALUES ($1, $2)
            RETURNING id, owner_id, title, last_message_at, status as "status!: DBConversationStatus"
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
    ) -> Result<Vec<DBConversation>, sqlx::Error> {
        sqlx::query_as!(
            DBConversation,
            r#"
            SELECT id, owner_id, title, last_message_at as "last_message_at:DateTime<Utc>", status as "status:_"
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
        content: String,
        role: DBMessageRole,
    ) -> Result<DBMessage, sqlx::Error> {
        sqlx::query_as!(
            DBMessage,
            r#"
                WITH new_message AS (
                INSERT INTO chat.messages (conversation_id, content, role)
                VALUES ($1, $2, $3)
                RETURNING id, conversation_id, content, role as "role!: DBMessageRole", created_at
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
            role as DBMessageRole
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_conversation_messages(
        &self,
        conversation_id: i32,
    ) -> Result<Vec<DBMessage>, sqlx::Error> {
        sqlx::query_as!(
            DBMessage,
            r#"
            SELECT id, conversation_id, content, role as "role!: DBMessageRole", created_at as "created_at:DateTime<Utc>"
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
