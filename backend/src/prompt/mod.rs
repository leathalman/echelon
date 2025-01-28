mod tests;

const INSTRUCTION: &str = "You are a helpful assistant. \
Provide answers using only the given context. \
Feel free to use as much information in the given context as you want.";

#[derive(Clone)]
pub struct Prompt {
    // what is passed to the model (instruction + context + question)
    instruction: String,
    context: Option<String>,
    question: Option<String>,
}

impl Prompt {
    pub fn new() -> Self {
        Prompt {
            instruction: INSTRUCTION.to_string(),
            context: None,
            question: None,
        }
    }

    pub fn instruction(mut self, instruction: String) -> Self {
        self.instruction = instruction;
        self
    }

    pub fn context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    pub fn question(mut self, question: String) -> Self {
        self.question = Some(question);
        self
    }

    pub fn to_string(self) -> String {
        let context = if let Some(val) = self.context {
            val
        } else {
            "None".to_string()
        };

        let question = if let Some(val) = self.question {
            val
        } else {
            "None".to_string()
        };

        format!(
            "{}\n\
        Context: {}\n\
        Question: {}",
            self.instruction, context, question
        )
    }
}
