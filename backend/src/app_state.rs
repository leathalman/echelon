use crate::config::Config;
use crate::llm::ollama::OllamaAdapter;
use crate::storage::postgres::RelationalStorage;
use crate::storage::qdrant::QdrantAdapter;

pub struct AppState {
    pub relational_storage: RelationalStorage,
    pub vector_storage: QdrantAdapter,
    pub llm: OllamaAdapter,
    pub config: Config,
}
