use iced::{widget::button, Element, Theme};

use crate::Message;

pub fn rounded_button<'a>(
    content: &'a str,
    on_press: Message,
    button_style: impl Fn(&Theme, button::Status) -> button::Style + 'a,
) -> Element<'a, Message> {
    button(content)
        .on_press(on_press)
        .style(button_style)
        .into()
}
