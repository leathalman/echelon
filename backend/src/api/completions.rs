use std::convert::Infallible;
use crate::app_state::AppState;
use crate::llm::inference::InferenceRequest;
use crate::llm::prompt::{Instruction, Prompt};
use crate::vectorization::embedding::embed;
use crate::storage::model::{DBMessageRole, DBUser};
use crate::storage::vector::VectorStorage;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Sse};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use axum::response::sse::Event;
use futures_util::{stream, Stream, StreamExt};
use ollama_rs::generation::completion::request::GenerationRequest;
use tracing::{error};

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

// TODO: why not grab collection from user profile instead of passing it via JSON?

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

    // info!("PROMPT:\n\n{:?}\n\n", prompt);

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

/// GET /api/completions/stream -> JWT required
pub async fn completion_streaming_handler(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<DBUser>,
    Json(payload): Json<CreateCompletionSchema>,
) -> Result<Sse<impl Stream<Item=Result<Event, Infallible>>>, (StatusCode, Json<serde_json::Value>)> {
    let user_queries: Vec<String> = payload
        .messages
        .iter()
        .filter(|message| matches!(message.role, DBMessageRole::User))
        .map(|message| message.content.clone())
        .collect();

    let user_queries_flattened = user_queries.join(" ");

    let user_queries_embedded = match embed(user_queries_flattened) {
        Ok(embedded) => embedded,
        Err(e) => {
            error!("Failed to embed user queries: {}", e.to_string());
            let error_response = json!({
                "message": "Unable to process completion request",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

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

    // Convert our prompt to the format needed for ollama
    let model_name = state.llm.model().to_string();
    let prompt_str = match prompt.instruction {
        Instruction::RAG => prompt.to_string_rag(),
        Instruction::Title => prompt.to_string_title(),
    };


    // Create the stream
    let stream_result = state.llm.client.generate_stream(
        GenerationRequest::new(model_name, prompt_str)
    ).await;

    match stream_result {
        Ok(ollama_stream) => {
            // Create a new stream that will transform the Ollama stream into SSE events
            let event_stream = stream::unfold(ollama_stream, |mut stream| async move {
                match stream.next().await {
                    Some(result) => {
                        match result {
                            Ok(responses) => {
                                if responses.is_empty() {
                                    // Empty response, continue to next chunk
                                    Some((Ok(Event::default().data("")), stream))
                                } else {
                                    // Process the first response and leave the rest for later
                                    let resp = &responses[0];
                                    Some((Ok(Event::default().data(&resp.response)), stream))
                                }
                            }
                            Err(e) => {
                                // Handle error in the stream
                                error!("Error in ollama stream: {}", e);
                                Some((
                                    Ok(Event::default()
                                        .event("error")
                                        .data(format!("Error: {}", e))),
                                    stream,
                                ))
                            }
                        }
                    }
                    None => None, // Stream is done
                }
            });

            Ok(Sse::new(event_stream))
        }
        Err(e) => {
            error!("Failed to create stream: {}", e.to_string());
            let error_response = json!({
                "message": "Unable to create completion stream",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
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