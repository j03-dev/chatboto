use iced::Task;
use rusql_alchemy::prelude::async_trait;

use crate::commons::types::AIMessage;
use crate::commons::utils::{content_builder, http};
use crate::{impl_model, perform_task, Message};
use std::fmt;
use std::str::FromStr;

#[async_trait]
pub trait Gemini {
    const URL: &'static str = "https://generativelanguage.googleapis.com/v1beta/models";
    const VERSION: &'static str;

    async fn ask(&self, prompt: String, history: Vec<AIMessage>) -> anyhow::Result<String> {
        let url = format!(
            "{}/{}:generateContent?key={}",
            Self::URL,
            Self::VERSION,
            self.api_key()
        );

        let mut contents: Vec<_> = history
            .into_iter()
            .map(|msg| content_builder(&msg.role, &msg.content))
            .collect();

        contents.push(content_builder("user", &prompt));

        let request_body = serde_json::json!({"contents": contents});

        let response: serializer::Response = http::post(&url, request_body, None).await?;

        let mut output = String::new();

        if let Some(candidate) = response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                output = part.text.clone();
            }
        }

        Ok(output)
    }

    fn api_key(&self) -> &str;

    fn set_api_key(&mut self, key: &str);
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

impl_model!(Gemini, GeminiV15Flash, "gemini-1.5-flash");
impl_model!(Gemini, GeminiV15Pro, "gemini-1.5-pro");
impl_model!(Gemini, GeminiV20Flash, "gemini-2.0-flash");
impl_model!(Gemini, GeminiV20Pro, "gemini-2.0-pro");
impl_model!(Gemini, GeminiV25Flash, "gemini-2.5-flash");
impl_model!(Gemini, GeminiV25Pro, "gemini-2.5-pro");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeminiModel {
    GeminiV15Flash(GeminiV15Flash),
    GeminiV15Pro(GeminiV15Pro),
    GeminiV20Flash(GeminiV20Flash),
    GeminiV20Pro(GeminiV20Pro),
    GeminiV25Flash(GeminiV25Flash),
    GeminiV25Pro(GeminiV25Pro),
}

impl GeminiModel {
    pub fn ask(self, api_key: String, prompt: String, history: Vec<AIMessage>) -> Task<Message> {
        match self {
            GeminiModel::GeminiV15Flash(g) => perform_task!(g, &api_key, prompt, history),
            GeminiModel::GeminiV15Pro(g) => perform_task!(g, &api_key, prompt, history),
            GeminiModel::GeminiV20Flash(g) => perform_task!(g, &api_key, prompt, history),
            GeminiModel::GeminiV20Pro(g) => perform_task!(g, &api_key, prompt, history),
            GeminiModel::GeminiV25Flash(g) => perform_task!(g, &api_key, prompt, history),
            GeminiModel::GeminiV25Pro(g) => perform_task!(g, &api_key, prompt, history),
        }
    }
}

impl FromStr for GeminiModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gemini-1.5-flash" => Ok(GeminiModel::GeminiV15Flash(GeminiV15Flash::default())),
            "gemini-1.5-pro" => Ok(GeminiModel::GeminiV15Pro(GeminiV15Pro::default())),
            "gemini-2.0-flash" => Ok(GeminiModel::GeminiV20Flash(GeminiV20Flash::default())),
            "gemini-2.0-pro" => Ok(GeminiModel::GeminiV20Pro(GeminiV20Pro::default())),
            "gemini-2.5-flash" => Ok(GeminiModel::GeminiV25Flash(GeminiV25Flash::default())),
            "gemini-2.5-pro" => Ok(GeminiModel::GeminiV25Pro(GeminiV25Pro::default())),
            _ => Err(()),
        }
    }
}

impl fmt::Display for GeminiModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = match self {
            GeminiModel::GeminiV15Flash(_) => "gemini-1.5-flash",
            GeminiModel::GeminiV15Pro(_) => "gemini-1.5-pro",
            GeminiModel::GeminiV20Flash(_) => "gemini-2.0-flash",
            GeminiModel::GeminiV20Pro(_) => "gemini-2.0-pro",
            GeminiModel::GeminiV25Flash(_) => "gemini-2.5-flash",
            GeminiModel::GeminiV25Pro(_) => "gemini-2.5-pro",
        };
        write!(f, "{}", version)
    }
}
