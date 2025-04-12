mod screens;
mod styles;
mod utils;
mod widgets;

use std::collections::HashMap;

use utils::gemini::Message as GeminiMessage;
use utils::mistral::Message as MistralMessage;

use iced::{widget::text_editor, Element, Task};
use screens::{chat_screen, setting_screen};

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

#[derive(Default, Debug, Clone)]
enum Screen {
    #[default]
    ChatScreen,
    #[allow(dead_code)]
    SettingScreen,
}

#[derive(Debug, Clone)]
enum Message {
    Submit,
    InputChanged(text_editor::Action),
    AIRespond(String),

    InputForm {
        key: String,
        value: String,
    },
    SaveSetting,

    Selected(AIChoice),

    #[allow(dead_code)]
    Route(Screen),
}

#[derive(Clone, Debug)]
enum MessageType {
    Sent,
    Received(AIChoice),
}

type Forms = HashMap<String, String>;

#[derive(Default)]
struct State {
    messages: Vec<(MessageType, String)>,
    ai_choice: Option<AIChoice>,
    gemini_history: Vec<GeminiMessage>,
    mistral_history: Vec<MistralMessage>,
    content: text_editor::Content,
    screen: Screen,
    forms: Forms,
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
        Message::InputChanged(action) => chat_screen::handle_user_input(state, action),
        Message::InputForm { key, value } => {
            input_form::get_input_form(&mut state.forms, key, value)
        }
        Message::Selected(choice) => chat_screen::handle_choice(state, choice),
        _ => Task::none(),
    }
}

fn main() -> iced::Result {
    dotenv::dotenv().ok();
    iced::run("ChatBoto", update, view)
}
