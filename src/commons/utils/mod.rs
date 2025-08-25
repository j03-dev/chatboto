pub mod http;


pub fn content_builder(role: &str, text: &str) -> serde_json::Value {
    serde_json::json!({"role": role, "parts": [serde_json::json!({"text": text})]})
}