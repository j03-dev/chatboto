use serde::Deserialize;
use serde_json::Value;

pub async fn fetch<T: for<'de> Deserialize<'de>>(url: &str, body: Value) -> T {
    let response = reqwest::blocking::Client::new()
        .post(url)
        .json(&body)
        .send()
        .expect("failed to send request");
    response.json().expect("failed to pase response ")
}
