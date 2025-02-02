use crate::llm::model::Model;
use crate::llm::ollama::OllamaAdapter;
use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait Inference {
    async fn generate(
        &self,
        request: InferenceRequest,
    ) -> Result<InferenceResponse, InferenceError>;
}

pub fn build(model: Model) -> impl Inference {
    match model {
        Model::Llama3_3b | Model::Llama3_11b | Model::Phi4 | Model::BespokeMinicheck => {
            OllamaAdapter::new(model)
        }
        Model::GPT4o => {
            todo!()
        }
    }
}

pub struct InferenceRequest {
    pub prompt: String,
    pub options: Option<InferenceOptions>,
}

impl InferenceRequest {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            options: None,
        }
    }

    pub fn options(mut self, options: InferenceOptions) -> Self {
        self.options = Some(options);
        self
    }
}

pub struct InferenceOptions {
    // Maximum number of tokens to generate (Default: -1, infinite generation)
    pub max_tokens: Option<i32>,
    // Size of the context window in tokens (Default: 32768)
    pub context_window: Option<u32>,
    // Sampling temperature - higher values increase creativity (Default: 0.8)
    pub temperature: Option<f32>,
}

impl Default for InferenceOptions {
    fn default() -> Self {
        Self {
            max_tokens: Some(-1),
            context_window: Some(32768),
            temperature: Some(0.8),
        }
    }
}

impl InferenceOptions {
    pub fn max_tokens(mut self, tokens: i32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    pub fn context_window(mut self, size: u32) -> Self {
        self.context_window = Some(size);
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }
}

#[derive(Debug)]
pub struct InferenceResponse {
    // The response of the completion. This can be the entire completion or only a token if the completion is streaming.
    pub content: String,
    // Number of tokens in the generated content
    pub token_count: Option<u16>,
    // Generation time in milliseconds
    pub generation_time: Option<u64>,
}

#[derive(Debug, Error)]
pub enum InferenceError {
    #[error("Inference error occurred: {0}")]
    Message(String),
}
