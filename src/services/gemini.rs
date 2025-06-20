use crate::commons::{
    types::{AIMessage, Gam, Version},
    utils::http,
};

use super::{build_content, AiService};

#[derive(Clone)]
pub struct GeminiService {
    pub version: Version,
    pub gam: Gam,
    pub api_key: String,
}

impl AiService for GeminiService {
    const URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
    type Response = serializer::Response;

    async fn ask(&self, text: String, history: Vec<AIMessage>) -> anyhow::Result<String> {
        let api = format!(
            "{}/gemini-{version}-{gam}:generateContent?key={api_key}",
            Self::URL,
            version = self.version,
            gam = self.gam,
            api_key = self.api_key
        );

        let mut contents: Vec<_> = history
            .iter()
            .map(|message| build_content(&message.role, &message.content))
            .collect();

        contents.push(build_content("user", &text));

        let body = serde_json::json!({"contents": contents});

        let response: Self::Response = http::post(&api, body, None).await?;

        let mut output = String::new();

        if let Some(candiate) = response.candidates.first() {
            if let Some(part) = candiate.content.parts.first() {
                output = part.text.clone();
            }
        }

        Ok(output)
    }
}

mod serializer {
    use serde::Deserialize;

    #[derive(Deserialize, Clone, Debug)]
    pub struct Part {
        pub text: String,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct Content {
        pub parts: Vec<Part>,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct Candidate {
        pub content: Content,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct Response {
        pub candidates: Vec<Candidate>,
    }
}
