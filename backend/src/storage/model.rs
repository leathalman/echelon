use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "chat.message_role", rename_all = "lowercase")]
#[derive(Clone)]
pub enum DBMessageRole {
    User,
    Assistant,
}

impl fmt::Display for DBMessageRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBMessageRole::User => write!(f, "user"),
            DBMessageRole::Assistant => write!(f, "assistant"),
        }
    }
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "chat.conversation_status", rename_all = "lowercase")]
pub enum DBConversationStatus {
    Active,
    Archived,
    Starred,
    System,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBUser {
    pub id: i32,
    pub student_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_login_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBConversation {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub last_message_at: DateTime<Utc>,
    pub status: DBConversationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBMessage {
    pub id: i32,
    pub conversation_id: i32,
    pub content: String,
    pub role: DBMessageRole,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Default)]
pub struct DBFilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
