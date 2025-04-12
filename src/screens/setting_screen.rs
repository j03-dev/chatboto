use iced::{
    widget::{column, row, text},
    Color, Element,
};

use crate::{
    styles,
    widgets::{button::rounded_button, input_form::input_form},
    Message, State,
};

pub fn setting(state: &State) -> Element<Message> {
    column![
        text("Setting")
            .center()
            .size(20)
            .color(Color::from(styles::GRAY)),
        input_form(
            "Mistral API key",
            state.forms.get("mistral").cloned().unwrap_or_default(),
            |value| Message::InputForm {
                key: "mistral".to_string(),
                value,
            },
            false
        ),
        input_form(
            "gemini",
            state.forms.get("gemini").cloned().unwrap_or_default(),
            |value| Message::InputForm {
                key: "gemini".to_string(),
                value
            },
            false
        ),
        row![
            rounded_button("Cancel", Message::SaveSetting, |_, status| {
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
