use crate::api::completions::ApiMessage;

const RAG_INSTRUCTION: &str = "You are Echelon, TCU's AI academic advising assistant designed to help students and faculty. \
You are talking with a student who has an academic profile. Everytime the user says 'I' they are referring to the academic profile. \
If courses are listed as '0.00' for 'Earned' or 'Points', assume these classes are currently being completed this semester. \
You should determine which year the student is currently enrolled as when you answer questions. \
Use the following pieces of retrieved context to answer the question with accurate information about TCU courses, programs, and policies. \
Format your responses in a clear, concise manner using markdown for readability when appropriate. \
When discussing courses, include course codes and relevant details such as prerequisites when available. \
Review the chat history before answering to maintain conversation continuity. \
If information is available in the context, provide specific, actionable guidance. \
If information is partially available, clearly indicate what is known and what might require further clarification. \
If you don't know or can't find the answer in the provided context, acknowledge this honestly and suggest how the user might find the information elsewhere (e.g., 'You may want to check with your academic advisor or the registrar's office for the most current information about this'). \
Never mention that you are using retrieved context or vector search in your responses.";
const TITLE_INSTRUCTION: &str = "Analyze the following conversation and generate a concise, descriptive title (maximum 8 words) that captures the main topic or purpose. The title should:
1. Identify the core subject matter or question being discussed.
2. Be specific enough to distinguish this conversation from others.
3. Avoid generic phrases like 'Discussion about' or 'Conversation regarding.'
4. Prioritize the earliest messages, which typically establish the conversation's purpose.
5. Capture any unique aspects, project names, or specific requests.
6. Be formatted in title case.
Output only the title as plain text, without any formatting, quotation marks, or additional explanations.";

#[derive(Debug)]
pub enum Instruction {
    RAG,
    Title,
}

#[derive(Debug)]
pub struct Prompt {
    pub history: Vec<ApiMessage>,
    pub profile: Option<String>,
    pub context: Option<String>,
    pub question: Option<String>,
    pub instruction: Instruction,
}

impl Prompt {
    pub fn new(
        history: Vec<ApiMessage>,
        profile: Option<String>,
        context: Option<String>,
        question: Option<String>,
        instruction: Instruction,
    ) -> Self {
        Prompt {
            history,
            profile,
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

        let profile = self.profile.unwrap_or_else(|| "".to_string());
        let context = self.context.unwrap_or_else(|| "".to_string());
        let question = self.question.unwrap_or_else(|| "".to_string());

        let instruction = match self.instruction {
            Instruction::RAG => RAG_INSTRUCTION,
            Instruction::Title => TITLE_INSTRUCTION,
        };

        format!(
            "Chat History:\n\
        {formatted_history}\
        Student's Academic Profile: \n\
        {}\n\
        Context:\n\
        {}\n\
        Question:\n\
        {}\n\
        Instruction:\n\
        {}
        ",
            profile, context, question, instruction
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
            Instruction::Title => TITLE_INSTRUCTION,
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
