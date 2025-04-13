use super::fetch::fetch;
use anyhow::Result;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::Deserialize;
use serde_json::json;

const URL: &str = "https://api.mistral.ai/v1/chat/completions";

#[derive(Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

pub async fn ask_mistral(text: String, history: Vec<Message>, api_key: String) -> Result<String> {
    let mut messages = history
        .iter()
        .map(|msg| {
            json!({
                "role": msg.role.clone(),
                "content": msg.content.clone()
            })
        })
        .collect::<Vec<_>>();

    messages.push(json!({
        "role": "user",
        "content": text
    }));

    let body = json!({
        "model": "mistral-large-latest",
        "messages": messages,
    });

    let mut headers = HeaderMap::new();
    let token = format!("Bearer {}", api_key);
    headers.insert(AUTHORIZATION, token.parse()?);

    let response: Response = fetch(URL, body, Some(headers)).await?;
    let mut output = String::new();
    if let Some(choice) = response.choices.first() {
        output = choice.message.content.clone();
    }

    Ok(output)
}
