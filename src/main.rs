mod screens;
mod styles;
mod utils;
mod widgets;

use std::collections::HashMap;

use screens::{chat_screen, setting_screen};
use utils::gemini::Message as GeminiMessage;
use utils::mistral::Message as MistralMessage;
use widgets::{nav, text_area};

use iced::{widget::text_editor, Element, Task};
use rusql_alchemy::prelude::*;

use crate::widgets::input_form;

#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
enum AIChoice {
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

#[derive(Model, FromRow, Clone)]
struct Config {
    #[field(primary_key = true)]
    id: Option<Integer>,
    ai_choice: Option<String>,
    gemini_apikey: Option<String>,
    mistral_apikey: Option<String>,
}

#[derive(Debug, Clone)]
enum Screen {
    ChatScreen,
    SettingScreen,
}

#[derive(Debug, Clone)]
enum Message {
    Submit,
    InputTextArea(text_editor::Action),
    AIRespond(String),

    InputForm {
        key: String,
        value: String,
    },
    SaveSetting,
    #[allow(dead_code)]
    SettingSaved(String),

    Selected(AIChoice),

    Route(Screen),
}

#[derive(Clone, Debug)]
enum MessageType {
    Sent,
    Received(AIChoice),
}

type Forms = HashMap<String, String>;

struct State {
    messages: Vec<(MessageType, String)>,
    ai_choice: Option<AIChoice>,
    gemini_history: Vec<GeminiMessage>,
    mistral_history: Vec<MistralMessage>,
    content: text_editor::Content,
    screen: Screen,
    forms: Forms,

    conn: Connection,
}

impl Default for State {
    fn default() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let (conn, config) = runtime.block_on(async {
            let database = Database::new().await.unwrap();
            database.migrate().await.ok();

            let conn = database.conn;

            let config = Config::get(kwargs!(id = 1), &conn).await.unwrap();

            (conn, config)
        });

        let forms = if let Some(config) = config {
            Forms::from([
                (
                    "mistral".to_string(),
                    config.mistral_apikey.unwrap_or_default(),
                ),
                (
                    "gemini".to_string(),
                    config.gemini_apikey.unwrap_or_default(),
                ),
            ])
        } else {
            Forms::new()
        };

        Self {
            messages: Vec::new(),
            ai_choice: Some(AIChoice::Gemini),
            gemini_history: Vec::new(),
            mistral_history: Vec::new(),
            content: text_editor::Content::new(),
            screen: Screen::ChatScreen,
            conn,
            forms,
        }
    }
}

fn view(state: &State) -> Element<Message> {
    match state.screen {
        Screen::ChatScreen => chat_screen::chat(state),
        Screen::SettingScreen => setting_screen::setting(state),
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Submit => chat_screen::action_submit(state),
        Message::AIRespond(response) => chat_screen::handle_ai_response(state, response),
        Message::InputTextArea(action) => text_area::handle_text_area_input(state, action),
        Message::InputForm { key, value } => {
            input_form::get_input_form(&mut state.forms, key, value)
        }
        Message::Selected(choice) => chat_screen::handle_choice(state, choice),
        Message::Route(screen) => nav::router_pushed(state, screen),
        Message::SaveSetting => setting_screen::save_setting(state),
        _ => Task::none(),
    }
}

fn main() -> iced::Result {
    iced::run("ChatBoto", update, view)
}
