use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

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

pub fn ask_gemini(text: String) -> Result<Response> {
    let api_key = std::env::var("API_KEY")?;
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

    let response = reqwest::blocking::Client::new()
        .post(format!("{URL}{api_key}"))
        .json(&body)
        .send()?;
    Ok(response.json()?)
}
