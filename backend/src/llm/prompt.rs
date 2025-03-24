use crate::api::completions::ApiMessage;

const RAG_INSTRUCTION: &str = "You are Echelon, TCU's AI academic advising assistant designed to help students and faculty. \
Use the following pieces of retrieved context to answer the question with accurate information about TCU courses, programs, and policies. \
Format your responses in a clear, concise manner using markdown for readability when appropriate. \
When discussing courses, include course codes and relevant details such as prerequisites when available. \
Review the chat history before answering to maintain conversation continuity. \
If information is available in the context, provide specific, actionable guidance. \
If information is partially available, clearly indicate what is known and what might require further clarification. \
If you don't know or can't find the answer in the provided context, acknowledge this honestly and suggest how the user might find the information elsewhere (e.g., 'You may want to check with your academic advisor or the registrar's office for the most current information about this'). \
Never mention that you are using retrieved context or vector search in your responses.";
const TITLE_INSTRUCTION: &str = "Analyze the following conversation and create a concise, descriptive title (maximum 8 words) that captures the main topic or purpose. The title should: \
1. Identify the core subject matter or question being discussed \
2. Be specific enough to distinguish this conversation from others \
3. Avoid generic phrases like 'Discussion about' or 'Conversation regarding' \
4. Prioritize the earliest messages which typically establish the conversation's purpose \
5. Capture any unique aspects, project names, or specific requests \
6. Be formatted in title case";

#[derive(Debug)]
pub enum Instruction {
    RAG,
    Title,
}

#[derive(Debug)]
pub struct Prompt {
    pub history: Vec<ApiMessage>,
    pub context: Option<String>,
    pub question: Option<String>,
    pub instruction: Instruction,
}

impl Prompt {
    pub fn new(history: Vec<ApiMessage>, context: Option<String>, question: Option<String>,
               instruction: Instruction) -> Self {
        Prompt {
            history,
            instruction,
            context,
            question,
        }
    }

    pub fn to_string_rag(self) -> String {
        let mut formatted_history = String::new();
        for message in &self.history {
            formatted_history.push_str(&format!(
                "[role: {}, content: {}]\n",
                message.role, message.content
            ));
        }

        let context = self.context.unwrap_or_else(|| { "".to_string() });
        let question = self.question.unwrap_or_else(|| { "".to_string() });

        let instruction = match self.instruction {
            Instruction::RAG => RAG_INSTRUCTION,
            Instruction::Title => TITLE_INSTRUCTION
        };

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
            context, question, instruction
        )
    }

    pub fn to_string_title(self) -> String {
        let mut formatted_history = String::new();
        for message in &self.history {
            formatted_history.push_str(&format!(
                "[role: {}, content: {}]\n",
                message.role, message.content
            ));
        }

        let instruction = match self.instruction {
            Instruction::RAG => RAG_INSTRUCTION,
            Instruction::Title => TITLE_INSTRUCTION
        };

        format!(
            "Chat History:\n\
        {formatted_history}\n\
        Instruction:\n\
        {}
        ",
            instruction
        )
    }
}
