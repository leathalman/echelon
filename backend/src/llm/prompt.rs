use crate::api::completions::ApiMessage;

const INSTRUCTION: &str = "You are a helpful academic advising assistant. \
Use the following pieces of retrieved context to answer the question without mentioning you were given context. \
If you don't know the answer, just say that you don't know.";

pub struct Prompt {
    history: Vec<ApiMessage>,
    context: String,
    question: String,
    instruction: String,
}

impl Prompt {
    pub fn new(history: Vec<ApiMessage>, context: String, question: String) -> Self {
        Prompt {
            history,
            instruction: INSTRUCTION.to_string(),
            context,
            question,
        }
    }

    pub fn to_string(self) -> String {
        let mut formatted_history = String::new();
        for message in &self.history {
            formatted_history.push_str(&format!(
                "[role: {}, content: {}]\n",
                message.role, message.content
            ));
        }

        format!(
            "Chat History:\n\
        {formatted_history}\
        Context:\n\
        {}\n\
        Question:\n\
        {}\n\
        Instruction:\n\
        {}
        ",
            self.context, self.question, self.instruction
        )
    }
}
