use std::fs;
use std::path::Path;
use thiserror::Error;

pub struct Document {
    pub content: String,
    pub file_name: String,
}

impl Document {
    pub fn new(path: &Path) -> Result<Self, DocumentError> {
        let content = fs::read_to_string(path).map_err(|e| {
            DocumentError::Message(format!("Unable to read file at path: {}", e.to_string()))
        })?;

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                DocumentError::Message("Invalid or missing file name in path".to_string())
            })?
            .to_string();

        Ok(Self { content, file_name })
    }
}

#[derive(Debug, Error)]
pub enum DocumentError {
    #[error("DocumentError occurred: {0}")]
    Message(String),
}
