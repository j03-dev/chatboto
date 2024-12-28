mod fetch;
mod gemini;
mod mistral;

use gemini::ask_gemini;
use iced::{
    alignment::Horizontal,
    border::Radius,
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Border, Element, Length, Task,
};
use mistral::ask_mistral;

const BLUE_SKY: [f32; 3] = [0.8, 0.9, 1.0];
const GRAY: [f32; 3] = [0.9, 0.9, 0.9];

#[derive(Clone, Debug, Default)]
enum AIChoice {
    #[default]
    Gemini,
    Mistral,
    None,
}

#[derive(Default)]
struct ChatBoto {
    messages: Vec<(MessageType, String)>,
    input_value: String,
    show_menu: bool,
    ai_choice: AIChoice,
}

#[derive(Clone, Debug)]
enum Message {
    Submit,
    InputChanged(String),
    AiResponde(String),
    ToggleMenu(AIChoice),
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

        let menu = if self.show_menu {
            container(
                column![
                    button("Gemini")
                        .on_press(Message::ToggleMenu(AIChoice::Gemini))
                        .width(Length::Fill)
                        .padding(10)
                        .style(|_, status| custom_style::menu_button(status)),
                    button("Mistral")
                        .on_press(Message::ToggleMenu(AIChoice::Mistral))
                        .width(Length::Fill)
                        .padding(10)
                        .style(|_, status| custom_style::menu_button(status)),
                    button("Cancel")
                        .on_press(Message::ToggleMenu(AIChoice::None))
                        .width(Length::Fill)
                        .padding(10)
                        .style(|_, status| custom_style::cancel_menu_button(status))
                ]
                .spacing(10),
            )
            .style(|_: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(GRAY.into())),
                border: Border {
                    radius: Radius::from(10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .padding(20)
            .width(200)
            .height(Length::Shrink)
            .align_x(Horizontal::Right) // Position to the top-right
            .align_y(Alignment::Center)
        } else {
            container(button("setting").on_press(Message::ToggleMenu(AIChoice::None)))
                .height(Length::Shrink)
        };

        container(
            column![menu, chat_area, input_area]
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
                    let task = {
                        match self.ai_choice {
                            AIChoice::Gemini => Task::perform(
                                ask_gemini(self.input_value.clone()),
                                Message::AiResponde,
                            ),
                            AIChoice::Mistral => Task::perform(
                                ask_mistral(self.input_value.clone()),
                                Message::AiResponde,
                            ),
                            _ => Task::none(),
                        }
                    };

                    self.input_value.clear();
                    return task;
                }
                Task::none()
            }
            Message::AiResponde(response) => {
                match self.ai_choice {
                    AIChoice::Gemini => {
                        self.messages
                            .push((MessageType::Received, response.clone()));
                    }
                    AIChoice::Mistral => {
                        self.messages
                            .push((MessageType::Received, response.clone()));
                    }
                    AIChoice::None => (),
                }
                Task::none()
            }
            Message::InputChanged(value) => {
                self.input_value = value;
                Task::none()
            }
            Message::ToggleMenu(choice) => {
                self.show_menu = !self.show_menu;
                match choice {
                    AIChoice::Gemini => self.ai_choice = AIChoice::Gemini,
                    AIChoice::Mistral => self.ai_choice = AIChoice::Mistral,
                    AIChoice::None => (),
                }
                Task::none()
            }
        }
    }
}

mod custom_style {
    // styles.rs

    use iced::{border::Radius, widget::button, Background, Border, Color};

    /// Defines the style for a menu button based on its status.
    pub fn menu_button(status: button::Status) -> button::Style {
        match status {
            button::Status::Hovered => button::Style {
                background: Some(Background::Color([0.9, 0.9, 0.9].into())), // Lighter gray
                text_color: Color::from_rgb(0.1, 0.1, 0.1),                  // Darker text
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5), // Gray border
                    width: 2.0,                            // Thickness of bottom border
                    radius: Radius::from(0.0),             // No rounding
                },
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(Background::Color([0.7, 0.7, 0.7].into())), // Darker gray
                text_color: Color::WHITE, // White text when pressed
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5), // Gray border
                    width: 2.0,                            // Thickness of bottom border
                    radius: Radius::from(0.0),             // No rounding
                },
                ..Default::default()
            },
            _ => button::Style {
                background: Some(Background::Color([0.8, 0.8, 0.8].into())), // Default gray
                text_color: Color::from_rgb(0.2, 0.2, 0.2),                  // Dark gray text
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5), // Gray border
                    width: 2.0,                            // Thickness of bottom border
                    radius: Radius::from(0.0),             // No rounding
                },
                ..Default::default()
            },
        }
    }

    /// Defines the style for a cancel menu button based on its status.
    pub fn cancel_menu_button(status: button::Status) -> button::Style {
        match status {
            button::Status::Hovered => button::Style {
                background: Some(Background::Color([0.95, 0.7, 0.7].into())), // Soft red background
                text_color: Color::from_rgb(0.2, 0.2, 0.2),                   // Dark gray text
                border: Border {
                    color: Color::from_rgb(0.7, 0.5, 0.5), // Soft red border
                    width: 2.0,                            // Thickness of bottom border
                    radius: Radius::from(0.0),             // No rounding
                },
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(Background::Color([0.85, 0.5, 0.5].into())), // Slightly darker red background
                text_color: Color::WHITE,                                     // White text
                border: Border {
                    color: Color::from_rgb(0.6, 0.4, 0.4), // Darker red border
                    width: 2.0,                            // Thickness of bottom border
                    radius: Radius::from(0.0),             // No rounding
                },
                ..Default::default()
            },
            _ => button::Style {
                background: Some(Background::Color([0.9, 0.6, 0.6].into())), // Default soft red background
                text_color: Color::from_rgb(0.2, 0.2, 0.2),                  // Dark gray text
                border: Border {
                    color: Color::from_rgb(0.7, 0.4, 0.4), // Soft red border
                    width: 2.0,                            // Thickness of bottom border
                    radius: Radius::from(0.0),             // No rounding
                },
                ..Default::default()
            },
        }
    }
}

fn main() -> iced::Result {
    dotenv::dotenv().ok();
    iced::run("ChatBoto", ChatBoto::update, ChatBoto::view)
}
