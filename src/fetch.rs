use anyhow::Result;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::Value;

pub fn fetch<T: for<'de> Deserialize<'de>>(
    url: &str,
    body: Value,
    headers: Option<HeaderMap>,
) -> Result<T> {
    let client = reqwest::blocking::Client::new();
    let mut request = client.post(url).json(&body);
    if let Some(headers) = headers {
        request = request.headers(headers);
    }
    let response = request.send()?;
    Ok(response.json()?)
}
