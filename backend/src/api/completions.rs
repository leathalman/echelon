use crate::app_state::AppState;
use crate::llm::inference::InferenceRequest;
use crate::llm::prompt::{Instruction, Prompt};
use crate::processing::embedding::embed;
use crate::storage::model::{DBMessageRole, DBUser};
use crate::storage::vector::VectorStorage;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiMessage {
    pub role: DBMessageRole,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCompletionSchema {
    pub messages: Vec<ApiMessage>,
    pub collection: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTitleCompletionSchema {
    pub messages: Vec<ApiMessage>,
}

/// POST /api/completions -> JWT required
// TODO: error handling is horrible in this method, please fix.
// TODO: generalize error message return (look at auth for this)
pub async fn completion_new_handler(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<DBUser>,
    Json(payload): Json<CreateCompletionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_queries: Vec<String> = payload
        .messages
        .iter()
        .filter(|message| matches!(message.role, DBMessageRole::User))
        .map(|message| message.content.clone())
        .collect();

    let user_queries_flattened = user_queries.join(" ");

    let user_queries_embedded = embed(user_queries_flattened).unwrap();

    let vector_search_result = match state
        .vector_storage
        .query(&payload.collection, user_queries_embedded)
        .await {
        Ok(result) => result,
        Err(e) => {
            error!("Failed on vector search: {}", e.to_string());
            let error_response = json!({
                "message": "Unable to create completion",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    // TODO: look into the score of what is returned...
    // add five highest rated results from VecDB to context for query
    let mut context = String::new();
    vector_search_result
        .points
        .into_iter()
        .take(5)
        .for_each(|point| context.push_str(&point.content));

    let profile = user.academic_profile.unwrap_or_else(|| "".to_string());

    let prompt = Prompt::new(
        payload.messages,
        Some(profile),
        Some(context),
        Some(user_queries.last().unwrap().to_string()),
        Instruction::RAG,
    );

    info!("PROMPT:\n\n{:?}\n\n", prompt);

    let completion = state
        .llm
        .generate(InferenceRequest::new(prompt))
        .await
        .unwrap();

    let json_response = json!({
        "content": completion.content,
        "generation_time": completion.generation_time,
        "token_count": completion.token_count
    });

    Ok(Json(json_response))
}

/// POST /api/completions/title -> JWT required
// TODO: using unwrap... server will crash if completion is not executed properly
pub async fn completion_new_title_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTitleCompletionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let prompt = Prompt {
        history: payload.messages,
        profile: None,
        context: None,
        question: None,
        instruction: Instruction::Title,
    };

    let completion = state
        .llm
        .generate(InferenceRequest::new(prompt))
        .await
        .unwrap();

    let json_response = json!({
        "content": completion.content,
        "generation_time": completion.generation_time,
        "token_count": completion.token_count
    });

    Ok(Json(json_response))
}






















