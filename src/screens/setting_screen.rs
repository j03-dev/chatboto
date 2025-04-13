use iced::{
    widget::{column, row, text},
    Color, Element, Task,
};
use rusql_alchemy::prelude::*;

use crate::{
    styles,
    widgets::{button::rounded_button, input_form::input_form},
    Config, Message, Screen, State,
};

pub fn setting(state: &State) -> Element<Message> {
    column![
        text("Setting").center().size(20).color(Color::BLACK),
        input_form(
            "Mistral API key",
            "mistral",
            &state.forms,
            |value| Message::InputForm {
                key: "mistral".to_string(),
                value,
            },
            true
        ),
        input_form(
            "Gemini API key",
            "gemini",
            &state.forms,
            |value| Message::InputForm {
                key: "gemini".to_string(),
                value
            },
            true
        ),
        row![
            rounded_button("Cancel", Message::Route(Screen::ChatScreen), |_, status| {
                styles::danger_button(status)
            }),
            rounded_button("Save", Message::SaveSetting, |_, status| {
                styles::primary_button(status)
            }),
        ]
        .spacing(5)
    ]
    .spacing(10)
    .padding(20)
    .into()
}

pub fn save_setting(state: &mut State) -> Task<Message> {
    let mistral_apikey = state.forms.get("mistral").cloned();
    let gemini_apikey = state.forms.get("gemini").cloned();
    let conn = state.conn.clone();

    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime
        .block_on(async {
            if let Some(config) = Config::get(kwargs!(id = 1), &conn).await.unwrap() {
                Config {
                    gemini_apikey,
                    mistral_apikey,
                    ..config
                }
                .update(&conn)
                .await
            } else {
                Config {
                    gemini_apikey,
                    mistral_apikey,
                    ..Default::default()
                }
                .save(&conn)
                .await
            }
        })
        .ok();

    Task::none()
}
