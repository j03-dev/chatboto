use crate::fetch::fetch;
use anyhow::Result;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::Deserialize;
use serde_json::json;

const URL: &str = "https://api.mistral.ai/v1/chat/completions";

#[derive(Deserialize)]
struct Message {
    content: String,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

pub fn ask_mistral(text: String) -> Result<String> {
    let mistral_api_key = std::env::var("MISTRAL_API_KEY")?;
    let body = json!({
        "model": "mistral-large-latest",
        "messages": [json!({"role": "user", "content": text})],
    });
    let mut headers = HeaderMap::new();
    let token = format!("Bearer {}", mistral_api_key);
    headers.insert(AUTHORIZATION, token.parse()?);
    let response: Response = fetch(URL, body, Some(headers))?;
    let mut output = String::new();
    if let Some(choice) = response.choices.first() {
        output = choice.message.content.clone();
    }
    Ok(output)
}
