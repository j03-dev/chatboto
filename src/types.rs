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
pub enum AIChoice {
    #[default]
    Gemini,
    Mistral,
}

impl std::fmt::Display for AIChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            AIChoice::Gemini => "gemini",
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
