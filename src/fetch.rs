use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::Value;

pub async fn fetch<T: for<'de> Deserialize<'de>>(
    url: &str,
    body: Value,
    headers: Option<HeaderMap>,
) -> T {
    let client = reqwest::blocking::Client::new();
    let mut request = client.post(url).json(&body);
    if let Some(headers) = headers {
        request = request.headers(headers);
    }
    let response = request.send().expect("failed to send request");
    response.json().expect("failed to pase response ")
}
