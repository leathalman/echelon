use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, Clone)]
pub enum Model {
    #[strum(serialize = "llama3.2:latest")]
    Llama3_3b,
    #[strum(serialize = "llama3.2-vision:11b")]
    Llama3_11b,
    #[strum(serialize = "phi4:latest")]
    Phi4,
    #[strum(serialize = "gpt-4o-2024-08-06")]
    GPT4o,
    #[strum(serialize = "bespoke-minicheck")]
    BespokeMinicheck,
    #[strum(serialize = "mistral-small:24b")]
    Mistral24b,
    #[strum(serialize = "gemma3:12b")]
    Gemma3_12b,
}
