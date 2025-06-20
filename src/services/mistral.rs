use reqwest::header::{HeaderMap, AUTHORIZATION};

use crate::{commons::utils::http, services::build_content};

use super::AiService;

#[derive(Clone)]
pub struct MistralService {
    pub api_key: String,
}

impl AiService for MistralService {
    const URL: &str = "https://api.mistral.ai/v1/chat/completions";
    type Response = serializer::Response;

    async fn ask(
        &self,
        text: String,
        history: Vec<crate::commons::types::AIMessage>,
    ) -> anyhow::Result<String> {
        let mut messages: Vec<_> = history
            .iter()
            .map(|message| build_content(&message.role, &message.content))
            .collect();

        messages.push(build_content("user", &text));

        let body = serde_json::json!({
            "model": "mistral-large-latest",
            "messages": messages,
        });

        let mut headers = HeaderMap::new();
        let token = format!("Bearer {}", self.api_key);
        headers.insert(AUTHORIZATION, token.parse()?);

        let response: Self::Response = http::post(Self::URL, body, Some(headers)).await?;
        let mut output = String::new();
        if let Some(choice) = response.choices.first() {
            output = choice.message.content.clone();
        }

        Ok(output)
    }
}

mod serializer {
    use serde::Deserialize;

    use crate::commons::types::AIMessage;

    #[derive(Deserialize)]
    pub struct Choice {
        pub message: AIMessage,
    }

    #[derive(Deserialize)]
    pub struct Response {
        pub choices: Vec<Choice>,
    }
}
