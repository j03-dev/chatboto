use super::fetch::fetch;
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

const URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=";

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

#[derive(Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub async fn ask_gemini(text: String, history: Vec<Message>, api_key: String) -> Result<String> {
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

    let response: Response = fetch(&format!("{URL}{api_key}"), body, None).await?;

    let mut output = String::new();
    if let Some(candidate) = response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            output = part.text.clone();
        }
    }

    Ok(output)
}
