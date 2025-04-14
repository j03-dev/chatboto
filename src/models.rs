use rusql_alchemy::prelude::*;
use serde::Deserialize;

#[derive(Model, FromRow, Clone)]
pub struct Config {
    #[field(primary_key = true)]
    pub id: Option<Integer>,
    pub ai_choice: Option<String>,
    pub gemini_apikey: Option<String>,
    pub mistral_apikey: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}
