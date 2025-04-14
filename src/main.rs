mod components;
mod models;
mod screens;
mod services;
mod state;
mod styles;
mod types;
mod utils;

use components::{input_form, nav_bar, text_input};
use models::Config;
use screens::{chat_screen, setting_screen};

use iced::time::{self, Duration};
use iced::{Element, Subscription, Task};
use state::State;
use types::{AIChoice, Message, Screen};

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
