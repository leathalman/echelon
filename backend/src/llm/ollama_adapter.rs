use crate::llm::inference::{
    Inference, InferenceError, InferenceOptions, InferenceRequest, InferenceResponse,
};
use crate::llm::model::Model;
use async_trait::async_trait;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationResponse;
use ollama_rs::generation::options::GenerationOptions;
use ollama_rs::Ollama;
use std::default::Default;

pub struct OllamaAdapter {
    client: Ollama,
    model: Model,
}

impl OllamaAdapter {
    // Ollama lib panics if default() cannot be constructed.
    pub fn new(model: Model) -> impl Inference {
        Self {
            client: Ollama::default(),
            model,
        }
    }

    pub fn model(&self) -> &Model {
        &self.model
    }
}

impl InferenceRequest {
    fn to_generation_request(self, model: &Model) -> GenerationRequest {
        GenerationRequest::new(model.to_string(), self.prompt)
    }
}

impl InferenceResponse {
    fn new(response: GenerationResponse) -> InferenceResponse {
        let eval_duration = response.eval_duration.map(|duration| duration / 1_000_000);

        Self {
            content: response.response,
            eval_count: response.eval_count,
            eval_duration,
        }
    }
}

impl From<InferenceOptions> for GenerationOptions {
    fn from(options: InferenceOptions) -> Self {
        GenerationOptions::default()
            // all properties set by default constructor of InferenceOptions, unwrap should be safe here
            .num_predict(options.num_predict.unwrap())
            .num_ctx(options.num_ctx.unwrap())
            .temperature(options.temperature.unwrap())
    }
}

#[async_trait]
impl Inference for OllamaAdapter {
    async fn generate(
        &self,
        request: InferenceRequest,
    ) -> Result<InferenceResponse, InferenceError> {
        let generation_request = request
            .to_generation_request(self.model())
            .options(InferenceOptions::default().into());

        match self.client.generate(generation_request).await {
            Ok(response) => Ok(InferenceResponse::new(response)),
            Err(err) => Err(InferenceError::Message(err.to_string())),
        }
    }
}
