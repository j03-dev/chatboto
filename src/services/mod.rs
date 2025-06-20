use gemini::GeminiService;
use mistral::MistralService;

use crate::commons::types::{AIChoice, AIMessage};

pub mod gemini;
pub mod mistral;

pub trait AiService {
    const URL: &str;
    type Response;

    async fn ask(&self, text: String, history: Vec<AIMessage>) -> anyhow::Result<String>;
}

fn build_content(role: &str, text: &str) -> serde_json::Value {
    serde_json::json!({"role": role, "parts": [serde_json::json!({"text": text})]})
}

pub async fn ask_ai(
    choice: AIChoice,
    text: String,
    history: Vec<AIMessage>,
    api_key: String,
) -> anyhow::Result<String> {
    match choice {
        AIChoice::Gemini(version, gam) => {
            GeminiService {
                version,
                gam,
                api_key,
            }
            .ask(text, history)
            .await
        }
        AIChoice::Mistral => MistralService { api_key }.ask(text, history).await,
    }
}
