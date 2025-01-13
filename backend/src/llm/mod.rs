pub mod inference;
pub mod model;
mod ollama_adapter;

use crate::llm::inference::Inference;
use crate::llm::model::Model;
use crate::llm::ollama_adapter::OllamaAdapter;

pub fn build(model: Model) -> impl Inference {
    match model {
        Model::Llama3_3b | Model::Llama3_11b | Model::Phi4 => OllamaAdapter::new(model),
        Model::GPT4o => {
            todo!()
        }
    }
}
