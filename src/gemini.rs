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

pub async fn ask_gemini(text: String) -> Response {
    let api_key = std::env::var("API_KEY").expect("failed to get api key");
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

    fetch(&format!("{URL}{api_key}"), body).await
}
