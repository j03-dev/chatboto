use iced::time::Duration;
use iced::widget::text_editor;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    Submit,
    InputTextArea(text_editor::Action),
    AIRespond(String),

    InputForm {
        key: String,
        value: String,
    },
    SaveSetting,

    #[allow(clippy::enum_variant_names)]
    DisplayMessage {
        duration: Duration,
        msg: String,
    },
    Tick,

    Selected(AIChoice),

    Route(Screen),
}

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

impl std::fmt::Display for AIChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            AIChoice::Gemini(v, g) => &format!("gemini-{v}-{g}"),
            AIChoice::Mistral => "mistral",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub enum Screen {
    ChatScreen,
    SettingScreen,
}

#[derive(Clone, Debug)]
pub enum MessageType {
    Sent,
    Received(AIChoice),
}

pub type FormState = HashMap<String, String>;
