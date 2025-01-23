use crate::vectordb::vector_store::VectorStorePoint;
use fancy_regex::Regex;
use fastembed::{Embedding, TextEmbedding};
use std::default::Default;
use std::fs;
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug)]
pub struct TextChunk {
    pub uuid: Uuid,
    pub content: String,
    pub embedding: Embedding,
    pub name: String,
}

impl From<TextChunk> for VectorStorePoint {
    fn from(chunk: TextChunk) -> Self {
        VectorStorePoint {
            uuid: Some(chunk.uuid.to_string()),
            content: Some(chunk.content),
            embedding: Some(chunk.embedding),
            name: Some(chunk.name),
            score: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("Embedding error occurred: {0}")]
    Message(String),
}

pub fn process_file(path: &Path) -> Result<Vec<TextChunk>, EmbeddingError> {
    let markdown = fs::read_to_string(path).map_err(|e| {
        EmbeddingError::Message(format!("Unable to read file at path: {}", e.to_string()))
    })?;

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            EmbeddingError::Message("Invalid or missing file name in path".to_string())
        })?;

    let raw_chunks = chunk(markdown)?;
    let mut processed_chunks: Vec<TextChunk> = vec![];

    for (idx, chunk) in raw_chunks.iter().enumerate() {
        let name = format!("{}_{}", file_name, idx);

        processed_chunks.push(TextChunk {
            uuid: generate_uuid(&name),
            embedding: generate_embedding(chunk)?,
            content: chunk.clone(),
            name,
        })
    }

    Ok(processed_chunks)
}

// TODO: use lazy regex instead: https://stackoverflow.com/questions/70223794/split-string-in-rust-treating-consecutive-delimiters-as-one
fn chunk(str: String) -> Result<Vec<String>, EmbeddingError> {
    // REGEX: (?m)(?=^# (?!#))
    // NOTE: requires newline before title to work
    // # Title 1\n# Title 2 => ["", "# Title 1\n", "# Title 2"]
    // # Title 1# Title 2 => ["", "# Title 1# Title 2"]

    let reg = Regex::new(r"(?m)(?=^# (?!#))").map_err(|e| {
        EmbeddingError::Message(format!(
            "Failed to initialize regex builder: {}",
            e.to_string()
        ))
    })?;

    let chunks: Result<Vec<String>, EmbeddingError> = reg
        .split(&str)
        .map(|chunk| {
            chunk.map(|c| c.to_string()).map_err(|e| {
                EmbeddingError::Message(format!("Failed to chunk string: {}", e.to_string()))
            })
        })
        .filter(|result| match result {
            Ok(ref chunk) => !chunk.trim().is_empty(),
            Err(_) => true,
        })
        .collect();

    match chunks {
        Ok(c) => Ok(c),
        Err(e) => Err(EmbeddingError::Message(format!(
            "Failed to chunk string: {}",
            e.to_string()
        ))),
    }
}

// uses preset namespace
fn generate_uuid(name: &str) -> Uuid {
    println!("NAME IS: {}", name);
    Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes())
}

// TODO: initializes a new embedding model for each run, this can be improved
// TODO: runs single embedding at a time, slower than needed, batching would be better, esp since
fn generate_embedding(content: &str) -> Result<Embedding, EmbeddingError> {
    let model = TextEmbedding::try_new(Default::default()).map_err(|e| {
        EmbeddingError::Message(format!("Could not initialize embedding model: {}", e))
    })?;

    let embeddings = model
        .embed(vec![content], None)
        .map_err(|e| EmbeddingError::Message(format!("Failed to embed content: {}", e)))?;

    let embedding = embeddings
        .get(0)
        .cloned()
        .ok_or_else(|| EmbeddingError::Message("No embedding found".to_string()))?;

    Ok(embedding)
}

mod tests {
    use super::*;

    #[test]
    fn test_chunk_string_idx_0() {
        let markdown =
            "# Title 1\n## Subheading 1\nContent 1\n# Title 2\n## Subheading 2\nContent 2"
                .to_string();

        let result = chunk(markdown).unwrap();
        assert_eq!(
            result.get(0).cloned().unwrap(),
            "# Title 1\n## Subheading 1\nContent 1\n".to_string()
        )
    }

    #[test]
    fn test_chunk_string_idx_1() {
        let markdown =
            "# Title 1\n## Subheading 1\nContent 1\n# Title 2\n## Subheading 2# Title3Content 2"
                .to_string();

        let result = chunk(markdown).unwrap();
        assert_eq!(
            result.get(1).cloned().unwrap(),
            "# Title 2\n## Subheading 2# Title3Content 2".to_string()
        )
    }

    #[test]
    fn test_chunk_newline() {
        let markdown = "\n\n\n\n\n\n".to_string();

        let result = chunk(markdown).unwrap();
        let empty_vec: Vec<String> = vec![];

        assert_eq!(result, empty_vec)
    }
}
