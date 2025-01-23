#[cfg(test)]
mod tests {
    use crate::embeddings::process::chunk;

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
