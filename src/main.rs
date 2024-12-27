mod gemini;

use gemini::ask_gemini;
use iced::{
    alignment::Horizontal,
    border::Radius,
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Border, Element, Length,
};

#[derive(Default)]
struct ChatBoto {
    messages: Vec<(MessageType, String)>,
    input_value: String,
}

#[derive(Clone, Debug)]
enum Message {
    Submit,
    InputChanged(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum MessageType {
    Sent,
    Received,
}

impl ChatBoto {
    pub fn view(&self) -> Element<Message> {
        let chat_area = scrollable(
            column(self.messages.iter().map(|(message_type, content)| {
                let bubble = container(
                    text(content)
                        .size(16)
                        .width(Length::Shrink)
                        .align_x(Horizontal::Center),
                )
                .padding(10)
                .style(match message_type {
                    MessageType::Sent => |_: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color([0.8, 0.9, 1.0].into())),
                        border: Border {
                            radius: Radius::from(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MessageType::Received => |_: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color([0.9, 0.9, 0.9].into())),
                        border: Border {
                            radius: Radius::from(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                })
                .max_width(250);

                let spacer = Space::with_width(iced::Length::Fill);

                match message_type {
                    MessageType::Sent => row![spacer, bubble]
                        .spacing(10)
                        .align_y(Alignment::End)
                        .padding(20)
                        .into(),
                    MessageType::Received => row![bubble, spacer]
                        .spacing(10)
                        .align_y(Alignment::Start)
                        .padding(20)
                        .into(),
                }
            }))
            .spacing(10),
        )
        .height(Length::Fill);

        let input_area = row![
            text_input("Type your message...", &self.input_value)
                .on_input(Message::InputChanged)
                .padding(10)
                .size(16)
                .width(Length::Fill),
            button("Send").on_press(Message::Submit).padding(10),
        ]
        .spacing(10);

        container(
            column![chat_area, input_area]
                .spacing(10)
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .padding(10)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                if !self.input_value.trim().is_empty() {
                    self.messages
                        .push((MessageType::Sent, self.input_value.clone()));
                    if let Ok(response) = ask_gemini(self.input_value.clone()) {
                        for part in response.candidates[0].content.parts.clone() {
                            self.messages.push((MessageType::Received, part.text));
                        }
                    }
                    self.input_value.clear();
                }
            }
            Message::InputChanged(value) => {
                self.input_value = value;
            }
        }
    }
}

fn main() -> iced::Result {
    dotenv::dotenv().ok();
    iced::run("ChatBoto App", ChatBoto::update, ChatBoto::view)
}
