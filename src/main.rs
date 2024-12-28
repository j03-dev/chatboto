mod fetch;
mod gemini;

use gemini::{ask_gemini, Response};
use iced::{
    alignment::Horizontal,
    border::Radius,
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Border, Element, Length, Task,
};

const BLUE_SKY: [f32; 3] = [0.8, 0.9, 1.0];
const GRAY: [f32; 3] = [0.9, 0.9, 0.9];

#[derive(Default)]
struct ChatBoto {
    messages: Vec<(MessageType, String)>,
    input_value: String,
}

#[derive(Clone, Debug)]
enum Message {
    Submit,
    InputChanged(String),
    GeminiResponde(Response),
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
                        background: Some(iced::Background::Color(BLUE_SKY.into())),
                        border: Border {
                            radius: Radius::from(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MessageType::Received => |_: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(GRAY.into())),
                        border: Border {
                            radius: Radius::from(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                })
                .max_width(500);

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

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Submit => {
                if !self.input_value.trim().is_empty() {
                    self.messages
                        .push((MessageType::Sent, self.input_value.clone()));
                    let task = Task::perform(
                        ask_gemini(self.input_value.clone()),
                        Message::GeminiResponde,
                    );
                    self.input_value.clear();
                    return task;
                }
                Task::none()
            }
            Message::GeminiResponde(response) => {
                if let Some(candidate) = response.candidates.first() {
                    if let Some(part) = candidate.content.parts.first() {
                        self.messages
                            .push((MessageType::Received, part.text.clone()));
                    }
                }
                Task::none()
            }
            Message::InputChanged(value) => {
                self.input_value = value;
                Task::none()
            }
        }
    }
}

fn main() -> iced::Result {
    dotenv::dotenv().ok();
    iced::run("ChatBoto App", ChatBoto::update, ChatBoto::view)
}
