use iced::{
    widget::{column, container, row, scrollable, text, Column, Space},
    Alignment, Color, Element, Length,
};

use super::markdown::markdown;
use crate::commons::types::TypeAgent;
use crate::{
    commons::types::Role,
    styles::{self, AI_LABEL_COLOR}
    , Message,
};

pub fn chat_area<'l>(messages: Vec<(Role, String)>) -> Element<'l, Message> {
    container(
        scrollable(
            column(messages.iter().map(|(message_type, content)| {
                create_chat_bubble(message_type.clone(), content.clone())
            }))
            .spacing(10),
        )
        .height(Length::Fill),
    )
    .into()
}

fn create_chat_bubble<'l>(message_type: Role, content: String) -> Element<'l, Message> {
    let author = match message_type {
        Role::Agent(ref choice) => match choice {
            TypeAgent::Gemini => column!(text("@gemini").color(Color::from(AI_LABEL_COLOR))),
            TypeAgent::Mistral => {
                column!(text("@mistral").color(Color::from(AI_LABEL_COLOR)))
            }
        },
        _ => column!(),
    };

    match message_type {
        Role::User => row![
            Space::with_width(Length::Fill),
            bubble_message(author, content, message_type)
        ]
        .spacing(10)
        .align_y(Alignment::End)
        .padding(20)
        .into(),
        Role::Agent(_) => row![
            bubble_message(author, content, message_type),
            Space::with_width(Length::Fill)
        ]
        .spacing(10)
        .align_y(Alignment::Start)
        .padding(20)
        .into(),
    }
}

fn bubble_message(
    author: Column<'_, Message>,
    content: String,
    message_type: Role,
) -> Element<Message> {
    container(column![
        author,
        markdown(Box::leak(content.into_boxed_str()))
    ])
    .padding(10)
    .style(styles::bubble_message(message_type))
    .width(Length::Shrink)
    .into()
}
