use crate::error::InferenceError;
use async_trait::async_trait;

#[async_trait]
pub trait Inference {
    async fn generate(
        &self,
        request: InferenceRequest,
    ) -> Result<InferenceResponse, InferenceError>;
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
    // Maximum number of tokens to predict when generating text. (Default: -1, infinite generation)
    pub num_predict: Option<i32>,
    // Sets the size of the context window used to generate the next token. (Default: 4096)
    pub num_ctx: Option<u32>,
    // The temperature of the model. Increasing the temperature will make the model answer more creatively. (Default: 0.8)
    pub temperature: Option<f32>,
}

impl Default for InferenceOptions {
    fn default() -> Self {
        Self {
            num_predict: Some(-1),
            num_ctx: Some(4096),
            temperature: Some(0.8),
        }
    }
}

impl InferenceOptions {
    pub fn num_predict(mut self, num_predict: i32) -> Self {
        self.num_predict = Some(num_predict);
        self
    }

    pub fn num_ctx(mut self, num_ctx: u32) -> Self {
        self.num_ctx = Some(num_ctx);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

#[derive(Debug)]
pub struct InferenceResponse {
    // The response of the completion. This can be the entire completion or only a token if the completion is streaming.
    pub content: String,
    // Number of tokens in the response
    pub eval_count: Option<u16>,
    // Time spent in milliseconds generating the response
    pub eval_duration: Option<u64>,
}
