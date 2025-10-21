use iced::{
    widget::{column, row, text},
    Color, Element, Length,
};

use crate::{
    components::{button::rounded_button, forms, message::message},
    styles, Message, Screen, State,
};

pub fn setting_screen(state: &'_ State) -> Element<'_, Message> {
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
        forms::text_input::form(
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
        forms::text_input::form(
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
            rounded_button(
                "Cancel",
                Message::Change(Screen::ChatScreen),
                |_, status| { styles::danger_button(status) }
            ),
            rounded_button("Save", Message::SaveSetting, |_, status| {
                styles::primary_button(status)
            }),
        ]
        .spacing(5),
        message(&state.message, &state.message_status)
    ]
    .spacing(10)
    .padding(20)
    .into()
}

pub mod actions {
    use iced::Task;

    use crate::{
        commons::types::MessageStatus, components::message, schemas::Config, Message, State,
    };
    use rusql_alchemy::prelude::*;

    pub fn save(state: &mut State) -> Task<Message> {
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
        message::actions::trigger("success", MessageStatus::Success, 2)
    }
}
