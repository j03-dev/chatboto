mod fetch;
mod gemini;
mod mistral;
mod styles;

use std::future::ready;

use anyhow::Result;
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
    Gemini,
    #[default]
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
    fn view(&self) -> Element<Message> {
        let chat_area = self.render_chat_area();
        let input_area = self.render_input_area();
        let menu = self.render_menu();

        container(
            column![menu, chat_area, input_area]
                .spacing(10)
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .padding(10)
        .into()
    }

    fn render_chat_area(&self) -> Element<Message> {
        container(
            scrollable(
                column(self.messages.iter().map(|(message_type, content)| {
                    self.render_message(message_type.clone(), content.clone())
                }))
                .spacing(10),
            )
            .height(Length::Fill),
        )
        .into()
    }

    fn render_message(&self, message_type: MessageType, content: String) -> Element<Message> {
        let author = match message_type {
            MessageType::Received(ref choice) => match choice {
                AIChoice::Gemini => column!(text("@gemini").color(iced::color!(255, 0, 0))),
                AIChoice::Mistral => column!(text("@mistral").color(iced::color!(255, 0, 0))),
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
        .style(match message_type.clone() {
            MessageType::Sent => |_: &iced::Theme| styles::card(BLUE_SKY),
            MessageType::Received(_) => |_: &iced::Theme| styles::card(GRAY),
        })
        .max_width(500);

        let spacer = Space::with_width(Length::Fill);

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
    }

    fn render_input_area(&self) -> Element<Message> {
        row![
            text_input("Type your message...", &self.input_value)
                .on_input(Message::InputChanged)
                .padding(10)
                .size(16)
                .width(Length::Fill),
            button("Send")
                .style(|_, status| styles::primary_button(status))
                .on_press(Message::Submit)
                .padding(10),
        ]
        .spacing(10)
        .into()
    }

    fn render_menu(&self) -> Element<Message> {
        if self.show_menu {
            container(
                column![
                    button("Gemini")
                        .on_press(Message::ToggleMenu(AIChoice::Gemini))
                        .width(Length::Fill)
                        .padding(10)
                        .style(|_, status| styles::primary_button(status)),
                    button("Mistral")
                        .on_press(Message::ToggleMenu(AIChoice::Mistral))
                        .width(Length::Fill)
                        .padding(10)
                        .style(|_, status| styles::primary_button(status)),
                    button("Cancel")
                        .on_press(Message::ToggleMenu(AIChoice::None))
                        .width(Length::Fill)
                        .padding(10)
                        .style(|_, status| styles::danger_button(status))
                ]
                .spacing(10),
            )
            .style(|_: &iced::Theme| styles::card(GRAY))
            .padding(20)
            .width(200)
            .height(Length::Shrink)
            .align_x(Horizontal::Right)
            .align_y(Alignment::Center)
            .into()
        } else {
            container(
                button("setting")
                    .style(|_, status| styles::primary_button(status))
                    .on_press(Message::ToggleMenu(AIChoice::None)),
            )
            .height(Length::Shrink)
            .into()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Submit => self.handle_submit(),
            Message::AIRespond(response) => self.handle_ai_response(response),
            Message::InputChanged(value) => self.handle_input_change(value),
            Message::ToggleMenu(choice) => self.handle_toggle_menu(choice),
        }
    }

    fn handle_submit(&mut self) -> Task<Message> {
        if self.input_value.trim().is_empty() {
            return Task::none();
        }

        self.messages
            .push((MessageType::Sent, self.input_value.clone()));

        let task = match self.ai_choice {
            AIChoice::Gemini => Task::perform(
                ready(ask_gemini(self.input_value.clone())),
                Self::map_ai_response,
            ),
            AIChoice::Mistral => Task::perform(
                ready(ask_mistral(self.input_value.clone())),
                Self::map_ai_response,
            ),
            _ => Task::none(),
        };

        self.input_value.clear();
        task
    }

    fn handle_ai_response(&mut self, response: String) -> Task<Message> {
        match self.ai_choice {
            AIChoice::Gemini => {
                self.messages
                    .push((MessageType::Received(AIChoice::Gemini), response));
            }
            AIChoice::Mistral => {
                self.messages
                    .push((MessageType::Received(AIChoice::Mistral), response));
            }
            AIChoice::None => (),
        }
        Task::none()
    }

    fn handle_input_change(&mut self, value: String) -> Task<Message> {
        self.input_value = value;
        Task::none()
    }

    fn handle_toggle_menu(&mut self, choice: AIChoice) -> Task<Message> {
        self.show_menu = !self.show_menu;
        if !self.show_menu {
            self.ai_choice = choice;
        }

        Task::none()
    }

    fn map_ai_response(resp: Result<String>) -> Message {
        Message::AIRespond(resp.unwrap_or_else(|err| err.to_string()))
    }
}

fn main() -> iced::Result {
    dotenv::dotenv().ok();
    iced::run("ChatBoto", ChatBoto::update, ChatBoto::view)
}

#[cfg(test)]
mod test {
    use crate::gemini::ask_gemini;
    use crate::mistral::ask_mistral;

    #[test]
    fn test_ask_mistral_ai() {
        dotenv::dotenv().ok();
        let response = ask_mistral("Hello".to_string());
        println!("response {response:#?}");
        assert!(response.is_ok())
    }

    #[test]
    fn test_ask_gemini() {
        dotenv::dotenv().ok();
        let response = ask_gemini("Hello".to_string());
        println!("response {response:#?}");
        assert!(response.is_ok())
    }
}
