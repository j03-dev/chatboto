use crate::models::gemini::*;
use crate::models::mistral::*;

use serde::Deserialize;
use std::fmt::Formatter;
use std::{collections::HashMap, fmt, str::FromStr};

#[derive(Deserialize, Clone)]
pub struct AIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Agent {
    Gemini(GeminiModel),
    Mistral(MistralModel),
}

impl Default for Agent {
    fn default() -> Self {
        Self::Gemini(GeminiModel::GeminiV15Flash(GeminiV15Flash::default()))
    }
}
impl FromStr for Agent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("gemini") {
            Ok(Agent::Gemini(GeminiModel::from_str(s)?))
        } else if s.starts_with("mistral") {
            Ok(Agent::Mistral(MistralModel::from_str(s)?))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Agent::Gemini(gemini_model) => write!(f, "{}", gemini_model),
            Agent::Mistral(mistral_model) => write!(f, "{}", mistral_model),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeAgent {
    Gemini,
    Mistral,
}

#[derive(Clone, Debug)]
pub enum Role {
    User,
    Agent(TypeAgent),
}

pub type FormState = HashMap<String, String>;

#[derive(Default, Debug, Clone)]
pub enum MessageStatus {
    Fail = 0,
    #[default]
    Success,
}
