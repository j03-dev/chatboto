use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

#[derive(Deserialize, Clone)]
pub struct AIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
pub enum Gam {
    #[default]
    Flash,
    Pro,
}

impl std::fmt::Display for Gam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let g = match self {
            Gam::Flash => "flash",
            Gam::Pro => "pro",
        };
        write!(f, "{}", g)
    }
}

#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    V1_5,
    #[default]
    V2_0,
    V2_5,
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version = match s {
            "1.5" => Version::V1_5,
            "2.0" => Version::V1_5,
            "2.5" => Version::V1_5,
            v => return Err(format!("{v}: is not candidate")),
        };
        Ok(version)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Version::V1_5 => "1.5",
            Version::V2_0 => "2.0",
            Version::V2_5 => "2.5",
        };
        write!(f, "{}", v)
    }
}

#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
pub enum AIChoice {
    Gemini(Version, Gam),
    #[default]
    Mistral,
}

impl FromStr for AIChoice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let aic = match s {
            "mistral" => AIChoice::Mistral,
            "gemini-1.5-flash" => AIChoice::Gemini(Version::V1_5, Gam::Flash),
            "gemini-1.5-pro" => AIChoice::Gemini(Version::V1_5, Gam::Pro),
            "gemini-2.0-flash" => AIChoice::Gemini(Version::V2_0, Gam::Flash),
            "gemini-2.0-pro" => AIChoice::Gemini(Version::V2_0, Gam::Pro),
            "gemini-2.5-flash" => AIChoice::Gemini(Version::V2_5, Gam::Flash),
            "gemini-2.5-pro" => AIChoice::Gemini(Version::V2_5, Gam::Pro),
            _ => return Err("ai choice should 'gemini' or 'mistral'"),
        };
        return Ok(aic);
    }
}

impl std::fmt::Display for AIChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            AIChoice::Gemini(v, g) => &format!("gemini-{v}-{g}"),
            AIChoice::Mistral => "mistral",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug)]
pub enum MessageType {
    Sent,
    Received(AIChoice),
}

pub type FormState = HashMap<String, String>;
