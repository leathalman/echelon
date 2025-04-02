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

pub fn chunk_by_words(
    content: String,
    max_words_per_chunk: usize,
    overlap_word_count: usize,
) -> Result<Vec<String>, ChunkError> {
    if content.is_empty() {
        return Ok(vec![]);
    }

    let words: Vec<&str> = content.split_whitespace().collect();

    if words.is_empty() {
        return Ok(vec![]);
    }

    // Ensure overlap is smaller than chunk size
    let effective_overlap = overlap_word_count.min(max_words_per_chunk - 1);
    let step_size = max_words_per_chunk - effective_overlap;

    let mut chunks = Vec::new();

    // Calculate how many full chunks we'll have
    let num_full_chunks = if words.len() <= max_words_per_chunk {
        1
    } else {
        1 + (words.len() - max_words_per_chunk + step_size - 1) / step_size
    };

    for i in 0..num_full_chunks {
        let start_position = i * step_size;
        // Don't go beyond the end of words
        if start_position >= words.len() {
            break;
        }

        let end_position = (start_position + max_words_per_chunk).min(words.len());
        let chunk_words = &words[start_position..end_position];
        let chunk = chunk_words.join(" ");

        chunks.push(chunk);
    }

    Ok(chunks)
}

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("ChunkError occurred: {0}")]
    Message(String),
}

#[cfg(test)]
mod tests {
    use crate::vectorization::chunk::{chunk, chunk_by_words};

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

    #[test]
    fn test_chunk_by_words_with_overlap() {
        let text = "One two three four five six seven eight nine ten eleven twelve";

        // Test with 4-word chunks and 2-word overlap
        let result = chunk_by_words(text.to_string(), 4, 2).unwrap();
        print!("{:?}", result);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "One two three four");
        assert_eq!(result[1], "three four five six");
        assert_eq!(result[2], "five six seven eight");
        assert_eq!(result[3], "seven eight nine ten");
        assert_eq!(result[4], "nine ten eleven twelve");

        // Test with 5-word chunks and 0-word overlap
        let result = chunk_by_words(text.to_string(), 5, 0).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "One two three four five");
        assert_eq!(result[1], "six seven eight nine ten");
        assert_eq!(result[2], "eleven twelve");

        // Test with larger chunk size than text
        let result = chunk_by_words(text.to_string(), 20, 5).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "One two three four five six seven eight nine ten eleven twelve");
    }
}
