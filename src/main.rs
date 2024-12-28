mod fetch;
mod gemini;
mod mistral;

mod styles;

use gemini::ask_gemini;
use iced::{
    alignment::Horizontal,
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Element, Length, Task,
};
use mistral::ask_mistral;
use styles::{BLUE_SKY, GRAY};

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
    AIRespond(String),
    ToggleMenu(AIChoice),
}

#[derive(Clone, Debug)]
enum MessageType {
    Sent,
    Received(AIChoice),
}

impl ChatBoto {
    pub fn view(&self) -> Element<Message> {
        let chat_area = scrollable(
            column(self.messages.iter().map(|(message_type, content)| {
                let author = match message_type {
                    MessageType::Received(choice) => match choice {
                        AIChoice::Gemini => column!(text("@gemini").color(iced::color!(255, 0, 0))),
                        AIChoice::Mistral => {
                            column!(text("@mistral").color(iced::color!(255, 0, 0)))
                        }
                        AIChoice::None => column!(),
                    },
                    _ => column!(),
                };
                let bubble = container(column![
                    author,
                    text(content)
                        .size(16)
                        .width(Length::Shrink)
                        .align_x(Horizontal::Center),
                ])
                .padding(10)
                .style(match message_type {
                    MessageType::Sent => |_: &iced::Theme| styles::card(BLUE_SKY),
                    MessageType::Received(_) => |_: &iced::Theme| styles::card(GRAY),
                })
                .max_width(500);

                let spacer = Space::with_width(iced::Length::Fill);

                match message_type {
                    MessageType::Sent => row![spacer, bubble]
                        .spacing(10)
                        .align_y(Alignment::End)
                        .padding(20)
                        .into(),
                    MessageType::Received(_) => row![bubble, spacer]
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
                        .style(|_, status| styles::menu_button(status)),
                    button("Mistral")
                        .on_press(Message::ToggleMenu(AIChoice::Mistral))
                        .width(Length::Fill)
                        .padding(10)
                        .style(|_, status| styles::menu_button(status)),
                    button("Cancel")
                        .on_press(Message::ToggleMenu(AIChoice::None))
                        .width(Length::Fill)
                        .padding(10)
                        .style(|_, status| styles::cancel_menu_button(status))
                ]
                .spacing(10),
            )
            .style(|_: &iced::Theme| styles::card(GRAY))
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
                            AIChoice::Gemini => {
                                Task::perform(ask_gemini(self.input_value.clone()), |resp| {
                                    let response = match resp {
                                        Ok(r) => r,
                                        Err(err) => err.to_string(),
                                    };
                                    Message::AIRespond(response)
                                })
                            }
                            AIChoice::Mistral => {
                                Task::perform(ask_mistral(self.input_value.clone()), |resp| {
                                    let response = match resp {
                                        Ok(r) => r,
                                        Err(err) => err.to_string(),
                                    };
                                    Message::AIRespond(response)
                                })
                            }
                            _ => Task::none(),
                        }
                    };

                    self.input_value.clear();
                    return task;
                }
                Task::none()
            }
            Message::AIRespond(response) => {
                match self.ai_choice {
                    AIChoice::Gemini => {
                        self.messages
                            .push((MessageType::Received(AIChoice::Gemini), response.clone()));
                    }
                    AIChoice::Mistral => {
                        self.messages
                            .push((MessageType::Received(AIChoice::Mistral), response.clone()));
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

fn main() -> iced::Result {
    dotenv::dotenv().ok();
    iced::run("ChatBoto", ChatBoto::update, ChatBoto::view)
}
