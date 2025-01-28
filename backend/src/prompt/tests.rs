#[cfg(test)]
mod tests {
    use crate::prompt::Prompt;

    #[test]
    fn test_prompt_to_string_no_context_no_question() {
        let prompt = Prompt::new();
        let formatted_prompt = format!("{}\nContext: None\nQuestion: None", prompt.instruction);

        assert_eq!(formatted_prompt, prompt.to_string())
    }

    #[test]
    fn test_prompt_to_string() {
        let prompt = Prompt::new()
            .context("TCU is a private university in Fort Worth, TX.".to_string())
            .question("What is TCU?".to_string());

        let formatted_prompt = format!(
            "{}\nContext: TCU is a private university in Fort Worth, TX.\nQuestion: What is TCU?",
            prompt.instruction
        );

        assert_eq!(formatted_prompt, prompt.to_string())
    }
}
