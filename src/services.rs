use crate::{types::AIMessage, utils::gemini::ask_gemini, utils::mistral::ask_mistral, AIChoice};
use anyhow::Result;

pub async fn ask_ai(
    choice: AIChoice,
    text: String,
    history: Vec<AIMessage>,
    api_key: String,
) -> Result<String> {
    match choice {
        AIChoice::Gemini(v, g) => ask_gemini(v, g, text, history, api_key).await,
        AIChoice::Mistral => ask_mistral(text, history, api_key).await,
    }
}
