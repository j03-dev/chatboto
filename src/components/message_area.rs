use iced::{
    alignment::Horizontal,
    widget::{column, container, row, scrollable, text, Column, Space},
    Alignment, Color, Element, Length,
};

use crate::{
    styles::{self, AI_LABEL_COLOR, BLUE_SKY, GRAY},
    types::MessageType,
    AIChoice, Message,
};

pub fn chat_area<'l>(messages: Vec<(MessageType, String)>) -> Element<'l, Message> {
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

fn create_chat_bubble<'l>(message_type: MessageType, content: String) -> Element<'l, Message> {
    let author = match message_type {
        MessageType::Received(ref choice) => match choice {
            AIChoice::Gemini => column!(text("@gemini").color(Color::from(AI_LABEL_COLOR))),
            AIChoice::Mistral => {
                column!(text("@mistral").color(Color::from(AI_LABEL_COLOR)))
            }
        },
        _ => column!(),
    };

    match message_type {
        MessageType::Sent => row![
            Space::with_width(Length::Fill),
            bubble_message(author, content, message_type)
        ]
        .spacing(10)
        .align_y(Alignment::End)
        .padding(20)
        .into(),
        MessageType::Received(_) => row![
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
    message_type: MessageType,
) -> Element<Message> {
    container(column![
        author,
        text(content)
            .size(16)
            .width(Length::Shrink)
            .align_x(Horizontal::Center),
    ])
    .padding(10)
    .style(match message_type.clone() {
        MessageType::Sent => |_: &iced::Theme| styles::card(BLUE_SKY),
        MessageType::Received(_) => |_: &iced::Theme| styles::card(GRAY),
    })
    .width(Length::Shrink)
    .into()
}
