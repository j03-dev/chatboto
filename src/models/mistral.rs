use std::fmt;
use std::str::FromStr;

use crate::commons::types::AIMessage;
use crate::commons::utils::{content_builder, http};
use crate::{impl_model, perform_task, Message};
use iced::Task;
use reqwest::header::{HeaderMap, AUTHORIZATION};

pub trait Mistral {
    const URL: &'static str = "https://api.mistral.ai/v1/chat/completions";
    const VERSION: &'static str;

    async fn ask(&self, text: String, history: Vec<AIMessage>) -> anyhow::Result<String> {
        let mut messages: Vec<_> = history
            .iter()
            .map(|message| content_builder(&message.role, &message.content))
            .collect();

        messages.push(content_builder("user", &text));

        let request_body = serde_json::json!({
            "model": Self::VERSION,
            "messages": messages,
        });

        let mut headers = HeaderMap::new();
        let token = format!("Bearer {}", self.api_key());
        headers.insert(AUTHORIZATION, token.parse()?);

        let response: serializer::Response =
            http::post(Self::URL, request_body, Some(headers)).await?;

        let mut output = String::new();
        if let Some(choice) = response.choices.first() {
            output = choice.message.content.clone();
        }

        Ok(output)
    }

    fn api_key(&self) -> &str;

    fn set_api_key(&mut self, api_key: &str);
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

impl_model!(Mistral, MistralLargeLatest, "mistral-large-latest");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MistralModel {
    MistralLargeLatest(MistralLargeLatest),
}

impl MistralModel {
    pub fn ask(self, api_key: String, prompt: String, history: Vec<AIMessage>) -> Task<Message> {
        match self {
            MistralModel::MistralLargeLatest(m) => perform_task!(m, &api_key, prompt, history),
        }
    }
}

impl FromStr for MistralModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mistral-lager-latest" => Ok(MistralModel::MistralLargeLatest(
                MistralLargeLatest::default(),
            )),
            _ => Err(()),
        }
    }
}

impl fmt::Display for MistralModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MistralModel::MistralLargeLatest(_) => write!(f, "mistral-lager-latest"),
        }
    }
}
