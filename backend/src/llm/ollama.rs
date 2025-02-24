use crate::llm::inference::{
    InferenceError, InferenceOptions, InferenceRequest, InferenceResponse,
};
use crate::llm::model::Model;
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
    // Must download the models via command line before calling from this program.
    pub fn new(model: Model) -> Self {
        Self {
            client: Ollama::default(),
            model,
        }
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub(crate) async fn generate(
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

impl InferenceRequest {
    fn to_generation_request(self, model: &Model) -> GenerationRequest {
        GenerationRequest::new(model.to_string(), self.prompt.to_string())
    }
}

impl InferenceResponse {
    fn new(response: GenerationResponse) -> InferenceResponse {
        let generation_time = response.eval_duration.map(|duration| duration / 1_000_000);

        Self {
            content: response.response,
            token_count: response.eval_count,
            generation_time,
        }
    }
}

impl From<InferenceOptions> for GenerationOptions {
    fn from(options: InferenceOptions) -> Self {
        GenerationOptions::default()
            // all properties set by default constructor of InferenceOptions, unwrap should be safe here
            .num_predict(options.max_tokens.unwrap())
            .num_ctx(options.context_window.unwrap())
            .temperature(options.temperature.unwrap())
    }
}
