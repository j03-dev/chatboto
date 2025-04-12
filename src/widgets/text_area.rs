use iced::{
    border::Radius,
    keyboard::{self, key},
    widget::text_editor,
    Color, Element,
};

use crate::{styles::BLUE_SKY, Message};

pub fn text_area(content: &text_editor::Content) -> Element<Message> {
    text_editor(content)
        .placeholder("Type your message ...")
        .on_action(Message::InputChanged)
        .style(|theme, status| text_editor::Style {
            border: iced::Border {
                width: 2.0,
                color: Color::from(BLUE_SKY),
                radius: Radius::from(8.0),
            },
            ..text_editor::default(theme, status)
        })
        .key_binding(|event| {
            let text_editor::KeyPress {
                ref key, modifiers, ..
            } = event;
            match key {
                keyboard::Key::Named(key::Named::Enter) if modifiers.command() => {
                    Some(text_editor::Binding::Custom(Message::Submit))
                }
                _ => text_editor::Binding::from_key_press(event),
            }
        })
        .padding(10)
        .size(16)
        .into()
}
