mod commons;
mod components;
mod models;
mod screens;
mod services;
mod styles;

use std::str::FromStr;

use commons::types::{AIChoice, AIMessage, FormState, MessageType};
use components::{forms, nav_bar};

use iced::widget::text_editor;
use models::Config;
use rusql_alchemy::prelude::*;
use screens::{chat, setting};

use iced::time::{self, Duration};
use iced::{Element, Subscription, Task};

pub struct State {
    pub messages: Vec<(MessageType, String)>,
    pub ai_choice: Option<AIChoice>,
    pub gemini_history: Vec<AIMessage>,
    pub mistral_history: Vec<AIMessage>,
    pub content: text_editor::Content,
    pub screen: Screen,
    pub forms: FormState,
    pub conn: Connection,
    pub tick: Duration,
    pub timer_enabled: bool,
    pub message: String,
}

impl Default for State {
    fn default() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let (conn, config) = runtime.block_on(async {
            let database = Database::new().await.unwrap();
            database.migrate().await.ok();
            let conn = database.conn;
            let config = Config::get(kwargs!(id == 1), &conn).await.unwrap();

            if config.is_none() {
                Config::default().save(&conn).await.unwrap();
            }

            (conn, config)
        });

        let forms: FormState = config
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
                cfg.ai_choice
                    .as_ref()
                    .map(|choice| AIChoice::from_str(&choice).unwrap())
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

#[derive(Debug, Clone)]
pub enum Screen {
    ChatScreen,
    SettingScreen,
}

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
    Change(Screen),
}
fn view(state: &State) -> Element<Message> {
    match state.screen {
        Screen::ChatScreen => chat::chat_screen(state),
        Screen::SettingScreen => setting::setting_screen(state),
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Submit => chat::actions::submit(state),
        Message::AIRespond(response) => chat::actions::push_response(state, response),
        Message::InputTextArea(action) => forms::text_area::actions::input(state, action),
        Message::InputForm { key, value } => {
            forms::text_input::actions::input(&mut state.forms, key, value)
        }
        Message::Selected(choice) => chat::actions::select(state, choice),
        Message::Change(screen) => nav_bar::actions::change(state, screen),
        Message::SaveSetting => setting::actions::save(state),
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
