use iced::{border::Radius, widget::text_input, Color, Element, Task};

use crate::{styles::BLUE_SKY, FormState, Message};

pub fn input_form<'l>(
    placeholder: &str,
    key: &str,
    forms: &FormState,
    on_input: impl Fn(String) -> Message + 'l,
    is_secure: bool,
) -> Element<'l, Message> {
    text_input(placeholder, &forms.get(key).cloned().unwrap_or_default())
        .style(|theme, status| text_input::Style {
            border: iced::Border {
                width: 2.0,
                color: Color::from(BLUE_SKY),
                radius: Radius::from(8.0),
            },
            ..text_input::default(theme, status)
        })
        .secure(is_secure)
        .on_input(on_input)
        .padding(10)
        .size(16)
        .into()
}

pub fn get_input_form(forms: &mut FormState, key: String, value: String) -> Task<Message> {
    forms.insert(key, value);
    Task::none()
}
