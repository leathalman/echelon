const INSTRUCTION: &str = "You are a helpful academic advising assistant. \
Use the following pieces of retrieved context to answer the question without mentioning you were given context. \
If you don't know the answer, just say that you don't know.";

// TODO: forgot about chat history here...
#[derive(Clone)]
pub struct Prompt {
    // what is passed to the model (instruction + context + question)
    instruction: String,
    context: String,
    question: String,
}

impl Prompt {
    pub fn new(context: String, question: String) -> Self {
        Prompt {
            instruction: INSTRUCTION.to_string(),
            context,
            question,
        }
    }

    pub fn to_string(self) -> String {
        format!(
            "{}\n\
        Context: {}\n\
        Question: {}",
            self.instruction, self.context, self.question
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::llm::prompt::Prompt;

    #[test]
    fn test_prompt_to_string() {
        let context = "TCU is a private university in Fort Worth, TX.".to_string();
        let question = "What is TCU?".to_string();

        let prompt = Prompt::new(context, question);

        let formatted_prompt = format!(
            "{}\nContext: TCU is a private university in Fort Worth, TX.\nQuestion: What is TCU?",
            prompt.instruction
        );

        assert_eq!(formatted_prompt, prompt.to_string())
    }
}
