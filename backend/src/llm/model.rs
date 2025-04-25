use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, Clone)]
pub enum Model {
    #[strum(serialize = "llama3.2:latest")]
    Llama3_3b,
    #[strum(serialize = "qwen2.5:72b-instruct-q4_K_M")]
    Qwen,
    #[strum(serialize = "phi4:14b-q4_K_M")]
    Phi4,
    #[strum(serialize = "gpt-4o-2024-08-06")]
    GPT4o,
    #[strum(serialize = "bespoke-minicheck")]
    BespokeMinicheck,
    #[strum(serialize = "mistral-small:24b-instruct-2501-q4_K_M")]
    Mistral24b,
    #[strum(serialize = "gemma3:27b-it-q4_K_M")]
    Gemma3,
    #[strum(serialize = "mistral-small3.1:24b-instruct-2503-q4_K_M")]
    MistralSmall,
}
