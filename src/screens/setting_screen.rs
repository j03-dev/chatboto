use iced::{
    time::Duration,
    widget::{column, row, text},
    Color, Element, Length, Task,
};
use rusql_alchemy::prelude::*;

use crate::{
    components::{button::rounded_button, input_form::input_form},
    styles, Config, Message, Screen, State,
};

pub fn setting(state: &State) -> Element<Message> {
    column![
        text("Setting")
            .size(20)
            .color(Color::BLACK)
            .center()
            .width(Length::Fill),
        text("Mistral API key")
            .center()
            .size(13)
            .color(Color::BLACK),
        input_form(
            "Add your mistral api key here",
            "mistral",
            &state.forms,
            |value| Message::InputForm {
                key: "mistral".to_string(),
                value,
            },
            true
        ),
        text("Gemini API key").center().size(13).color(Color::BLACK),
        input_form(
            "Add your gemini api key here",
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
        .spacing(5),
        text(&state.message)
            .size(20)
            .color(Color::from([0.0, 0.5, 0.0]))
            .center()
            .width(Length::Fill),
    ]
    .spacing(10)
    .padding(20)
    .into()
}

pub fn save_setting(state: &mut State) -> Task<Message> {
    let mistral_apikey = state.forms.get("mistral").cloned();
    let gemini_apikey = state.forms.get("gemini").cloned();
    let conn = &state.conn;
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        if let Ok(Some(config)) = Config::get(kwargs!(id == 1), conn).await {
            Config {
                gemini_apikey,
                mistral_apikey,
                ..config
            }
            .update(conn)
            .await
            .ok();
        }
    });
    Task::done(Message::DisplayMessage {
        duration: Duration::from_secs(2),
        msg: "Sucesss".to_string(),
    })
}
