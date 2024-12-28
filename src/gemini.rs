use serde::Deserialize;
use serde_json::json;

use super::fetch::fetch;

const URL: &str =
    "https://generativelanguage.googleapis.com/v1/models/gemini-pro:generateContent?key=";

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

pub async fn ask_gemini(text: String) -> String {
    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("failed to get api key");
    let body = json!({
        "contents": vec![
            json!({
                "role": "user",
                "parts": vec![
                    json!({
                        "text": text
                    })
                ]
            })
        ]
    });

    let response: Response = fetch(&format!("{URL}{gemini_api_key}"), body, None).await;

    let mut output = String::new();
    if let Some(candidate) = response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            output = part.text.clone();
        }
    }

    output
}
