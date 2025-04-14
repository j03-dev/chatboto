use rusql_alchemy::prelude::*;

#[derive(Model, FromRow, Clone)]
pub struct Config {
    #[field(primary_key = true)]
    pub id: Option<Integer>,
    pub ai_choice: Option<String>,
    pub gemini_apikey: Option<String>,
    pub mistral_apikey: Option<String>,
}
