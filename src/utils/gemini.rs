use super::fetch::fetch;
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::json;

use crate::types::{AIMessage, Gam, Version};

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

pub async fn ask_gemini(
    v: Version,
    g: Gam,
    text: String,
    history: Vec<AIMessage>,
    api_key: String,
) -> Result<String> {
    let mut contents = history
        .iter()
        .map(|msg| {
            json!({
                "role": msg.role,
                "parts": [json!({"text": msg.content})]
            })
        })
        .collect::<Vec<_>>();

    contents.push(json!({
        "role": "user",
        "parts": [json!({"text": text})]
    }));

    let body = json!({ "contents": contents });

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-{v}-{g}:generateContent?key={api_key}");

    let response: Response = fetch(&url, body, None).await?;

    let mut output = String::new();
    if let Some(candidate) = response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            output = part.text.clone();
        }
    }

    Ok(output)
}
