pub mod inference;
pub mod model;
mod ollama_wrapper;

use crate::llm::inference::Inference;
use crate::llm::model::Model;
use crate::llm::ollama_wrapper::OllamaWrapper;

pub fn build(model: Model) -> impl Inference {
    match model {
        Model::Llama3_3b | Model::Llama3_11b | Model::Phi4 => OllamaWrapper::new(model),
        Model::GPT4o => {
            todo!()
        }
    }
}
