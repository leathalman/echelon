use fancy_regex::Regex;
use thiserror::Error;

// REGEX: (?m)(?=^# (?!#))
// NOTE: requires newline before title to work
// # Title 1\n# Title 2 => ["# Title 1\n", "# Title 2"]
// # Title 1# Title 2 => ["# Title 1# Title 2"]
const REGEX: &str = r"(?m)(?=^# (?!#))";

// only supports markdown header splitting
pub fn chunk(content: String) -> Result<Vec<String>, ChunkError> {
    let reg = Regex::new(REGEX).map_err(|e| {
        ChunkError::Message(format!(
            "Failed to initialize regex builder: {}",
            e.to_string()
        ))
    })?;

    let chunks: Result<Vec<String>, ChunkError> = reg
        .split(&content)
        .map(|chunk| {
            chunk.map(|c| c.to_string()).map_err(|e| {
                ChunkError::Message(format!("Failed to chunk string: {}", e.to_string()))
            })
        })
        .filter(|result| match result {
            Ok(ref chunk) => !chunk.trim().is_empty(),
            Err(_) => true,
        })
        .collect();

    match chunks {
        Ok(c) => Ok(c),
        Err(e) => Err(ChunkError::Message(format!(
            "Failed to chunk string: {}",
            e.to_string()
        ))),
    }
}

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("ChunkError occurred: {0}")]
    Message(String),
}

#[cfg(test)]
mod tests {
    use crate::processing::chunk::chunk;

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
