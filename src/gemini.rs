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

pub async fn ask_gemini(text: String) -> Result<String> {
    let gemini_api_key = std::env::var("GEMINI_API_KEY")?;
    let body = json!({
        "contents": [
            json!({
                "role": "user",
                "parts": [json!({"text": text})]
            })
        ]
    });

    let response: Response = fetch(&format!("{URL}{gemini_api_key}"), body, None).await?;

    let mut output = String::new();
    if let Some(candidate) = response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            output = part.text.clone();
        }
    }

    Ok(output)
}
