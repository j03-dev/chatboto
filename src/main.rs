mod components;
mod models;
mod screens;
mod services;
mod styles;
mod utils;

use std::collections::HashMap;

use components::{input_form, nav_bar, text_input};
use models::Config;
use screens::{chat_screen, setting_screen};

use iced::time::{self, Duration};
use iced::{widget::text_editor, Element, Subscription, Task};
use rusql_alchemy::prelude::*;

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

    #[allow(clippy::enum_variant_names)]
    DisplayMessage {
        duration: Duration,
        msg: String,
    },
    Tick,

    Selected(AIChoice),

    Route(Screen),
}

#[derive(Clone, Debug)]
enum MessageType {
    Sent,
    Received(AIChoice),
}

type FormState = HashMap<String, String>;

struct State {
    messages: Vec<(MessageType, String)>,
    ai_choice: Option<AIChoice>,
    gemini_history: Vec<models::Message>,
    mistral_history: Vec<models::Message>,
    content: text_editor::Content,
    screen: Screen,
    forms: FormState,
    conn: Connection,

    tick: Duration,
    timer_enabled: bool,

    message: String,
}

impl Default for State {
    fn default() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let (conn, config) = runtime.block_on(async {
            let database = Database::new().await.unwrap();
            database.migrate().await.ok();
            let conn = database.conn;
            let config = Config::get(kwargs!(id == 1), &conn).await.unwrap();
            (conn, config)
        });

        let forms = config
            .as_ref()
            .map(|cfg| {
                FormState::from([
                    (
                        "mistral".to_string(),
                        cfg.mistral_apikey.clone().unwrap_or_default(),
                    ),
                    (
                        "gemini".to_string(),
                        cfg.gemini_apikey.clone().unwrap_or_default(),
                    ),
                ])
            })
            .unwrap_or_default();

        let ai_choice = config
            .as_ref()
            .and_then(|cfg| {
                cfg.ai_choice.as_ref().map(|choice| match choice.as_str() {
                    "mistral" => AIChoice::Mistral,
                    "gemini" => AIChoice::Gemini,
                    _ => panic!("ai choice should 'gemini' or 'mistral'"),
                })
            })
            .unwrap_or_default();

        Self {
            messages: Vec::new(),
            ai_choice: Some(ai_choice),
            gemini_history: Vec::new(),
            mistral_history: Vec::new(),
            content: text_editor::Content::new(),
            screen: Screen::ChatScreen,
            conn,
            forms,
            timer_enabled: false,
            tick: Duration::default(),
            message: String::new(),
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
        Message::InputTextArea(action) => text_input::handle_text_area_input(state, action),
        Message::InputForm { key, value } => {
            input_form::get_input_form(&mut state.forms, key, value)
        }
        Message::Selected(choice) => chat_screen::handle_choice(state, choice),
        Message::Route(screen) => nav_bar::router_pushed(state, screen),
        Message::SaveSetting => setting_screen::save_setting(state),
        Message::DisplayMessage { duration, msg } => {
            state.timer_enabled = true;
            state.message = msg;
            state.tick = duration;
            Task::none()
        }
        Message::Tick => {
            if state.tick > Duration::default() {
                state.tick -= Duration::from_secs(1);
            } else {
                state.timer_enabled = false;
                state.message = String::new();
                state.tick = Duration::default();
            }
            Task::none()
        }
    }
}

fn subscription(state: &State) -> Subscription<Message> {
    if state.timer_enabled {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    } else {
        Subscription::none()
    }
}
fn main() -> iced::Result {
    iced::application("ChatBoto", update, view)
        .subscription(subscription)
        .run()
}
