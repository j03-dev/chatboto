mod fetch;
mod gemini;
mod mistral;
mod styles;

use gemini::{ask_gemini, Message as GeminiMessage};
use mistral::{ask_mistral, Message as MistralMessage};

use anyhow::Result;
use iced::{
    alignment::Horizontal,
    border::Radius,
    keyboard::{self, key},
    widget::{button, column, container, row, scrollable, text, text_editor, Space},
    Alignment, Color, Element, Length, Task,
};
use styles::{AI_LABEL_COLOR, BLUE_SKY, GRAY};

#[derive(Clone, Debug, Default, PartialEq)]
enum AIChoice {
    Gemini,
    #[default]
    Mistral,
    None,
}

#[derive(Default)]
struct ChatBoto {
    messages: Vec<(MessageType, String)>,
    show_menu: bool,
    ai_choice: AIChoice,
    gemini_history: Vec<GeminiMessage>,
    mistral_history: Vec<MistralMessage>,

    content: text_editor::Content,
}

#[derive(Clone, Debug)]
enum Message {
    Submit,
    InputChanged(text_editor::Action),
    AIRespond(String),
    ToggleMenu(AIChoice),
}

#[derive(Clone, Debug)]
enum MessageType {
    Sent,
    Received(AIChoice),
}

impl ChatBoto {
    const SPACING: u16 = 10;

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
                .spacing(Self::SPACING),
            )
            .height(Length::Fill),
        )
        .into()
    }

    fn render_message(&self, message_type: MessageType, content: String) -> Element<Message> {
        let author = match message_type {
            MessageType::Received(ref choice) => match choice {
                AIChoice::Gemini => column!(text("@gemini").color(Color::from(AI_LABEL_COLOR))),
                AIChoice::Mistral => {
                    column!(text("@mistral").color(Color::from(AI_LABEL_COLOR)))
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
        .style(match message_type.clone() {
            MessageType::Sent => |_: &iced::Theme| styles::card(BLUE_SKY),
            MessageType::Received(_) => |_: &iced::Theme| styles::card(GRAY),
        })
        .width(Length::Shrink);

        let spacer = Space::with_width(Length::Fill);

        match message_type {
            MessageType::Sent => row![spacer, bubble]
                .spacing(Self::SPACING)
                .align_y(Alignment::End)
                .padding(20)
                .into(),
            MessageType::Received(_) => row![bubble, spacer]
                .spacing(Self::SPACING)
                .align_y(Alignment::Start)
                .padding(20)
                .into(),
        }
    }

    fn render_input_area(&self) -> Element<Message> {
        row![
            container(
                text_editor(&self.content)
                    .placeholder("Type your message ...")
                    .on_action(Message::InputChanged)
                    .style(|theme, status| text_editor::Style {
                        border: iced::Border {
                            width: 2.0,
                            color: Color::from(BLUE_SKY),
                            radius: Radius::from(8.0),
                            ..Default::default()
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
            )
            .max_height(200),
            button("Send")
                .style(|_, status| styles::primary_button(status))
                .on_press(Message::Submit)
                .padding(10),
        ]
        .spacing(Self::SPACING)
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
                .spacing(Self::SPACING),
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
            Message::InputChanged(action) => {
                self.content.perform(action);
                Task::none()
            }
            Message::ToggleMenu(choice) => self.handle_toggle_menu(choice),
        }
    }

    fn handle_submit(&mut self) -> Task<Message> {
        let input_value = self.content.text();
        if input_value.trim().is_empty() {
            return Task::none();
        }

        self.messages.push((MessageType::Sent, input_value.clone()));

        let user_message = input_value.clone();

        let task = match self.ai_choice {
            AIChoice::Gemini => {
                self.gemini_history.push(GeminiMessage {
                    role: "user".to_string(),
                    content: user_message.clone(),
                });

                let history = self.gemini_history.clone();

                Task::perform(ask_gemini(user_message, history), Self::map_ai_response)
            }
            AIChoice::Mistral => {
                self.mistral_history.push(MistralMessage {
                    role: "user".to_string(),
                    content: user_message.clone(),
                });

                let history = self.mistral_history.clone();

                Task::perform(ask_mistral(user_message, history), Self::map_ai_response)
            }
            _ => Task::none(),
        };

        self.content = text_editor::Content::new();
        task
    }

    fn handle_ai_response(&mut self, response: String) -> Task<Message> {
        match self.ai_choice {
            AIChoice::Gemini => {
                self.messages
                    .push((MessageType::Received(AIChoice::Gemini), response.clone()));

                self.gemini_history.push(GeminiMessage {
                    role: "model".to_string(),
                    content: response,
                });
            }
            AIChoice::Mistral => {
                self.messages
                    .push((MessageType::Received(AIChoice::Mistral), response.clone()));

                self.mistral_history.push(MistralMessage {
                    role: "assistant".to_string(),
                    content: response,
                });
            }
            AIChoice::None => (),
        }
        Task::none()
    }

    fn handle_toggle_menu(&mut self, choice: AIChoice) -> Task<Message> {
        self.show_menu = !self.show_menu;
        if !self.show_menu && choice != AIChoice::None {
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
