use iced::{
    border::Radius,
    widget::{column, container, overlay::menu, pick_list, row, text_editor},
    Background, Color, Element, Length, Task,
};

use crate::{
    styles::{self, BLUE_SKY},
    utils::{
        gemini::{ask_gemini, Message as GeminiMessage},
        mistral::{ask_mistral, Message as MistralMessage},
    },
    widgets::{button::rounded_button, message_area::render_chat_area, nav, text_area::text_area},
    AIChoice, Message, MessageType, State,
};

pub fn chat(state: &State) -> Element<Message> {
    let choices = [AIChoice::Gemini, AIChoice::Mistral];
    column![
        nav::nav(),
        render_chat_area(state.messages.clone()),
        row![
            container(text_area(&state.content)).max_height(200),
            pick_list(choices, state.ai_choice, Message::Selected)
                .style(|theme, status| {
                    pick_list::Style {
                        placeholder_color: Color::BLACK,
                        border: iced::Border {
                            width: 2.0,
                            color: Color::from(BLUE_SKY),
                            radius: Radius::from(8.0),
                        },
                        ..pick_list::default(theme, status)
                    }
                })
                .menu_style(|theme| menu::Style {
                    border: iced::Border {
                        radius: Radius::from(8.0),
                        ..Default::default()
                    },
                    selected_background: Background::Color(Color::from([0.2, 0.6, 1.0])),
                    ..menu::default(theme)
                })
                .placeholder("Agents"),
            rounded_button("Send", Message::Submit, |_, status| styles::primary_button(
                status
            )),
        ]
        .spacing(10)
    ]
    .spacing(10)
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(10)
    .into()
}

pub fn handle_choice(state: &mut State, choice: AIChoice) -> Task<Message> {
    state.ai_choice = Some(choice);
    Task::none()
}

pub fn action_submit(state: &mut State) -> Task<Message> {
    let value = state.content.text();

    if value.trim().is_empty() {
        return Task::none();
    }

    state.messages.push((MessageType::Sent, value.clone()));

    let task = match state.ai_choice {
        Some(AIChoice::Gemini) => {
            state.gemini_history.push(GeminiMessage {
                role: "user".to_string(),
                content: value.clone(),
            });
            let api_key = state.forms.get("gemini").cloned().unwrap();
            Task::perform(
                ask_gemini(value, state.gemini_history.clone(), api_key),
                |resp| Message::AIRespond(resp.unwrap_or_else(|err| err.to_string())),
            )
        }
        Some(AIChoice::Mistral) => {
            state.mistral_history.push(MistralMessage {
                role: "user".to_string(),
                content: value.clone(),
            });
            let api_key = state.forms.get("mistral").cloned().unwrap();
            Task::perform(
                ask_mistral(value, state.mistral_history.clone(), api_key),
                |resp| Message::AIRespond(resp.unwrap_or_else(|err| err.to_string())),
            )
        }
        None => Task::none(),
    };

    state.content = text_editor::Content::new();
    task
}

pub fn handle_ai_response(state: &mut State, response: String) -> Task<Message> {
    match state.ai_choice {
        Some(AIChoice::Gemini) => {
            state
                .messages
                .push((MessageType::Received(AIChoice::Gemini), response.clone()));

            state.gemini_history.push(GeminiMessage {
                role: "model".to_string(),
                content: response,
            });
        }
        Some(AIChoice::Mistral) => {
            state
                .messages
                .push((MessageType::Received(AIChoice::Mistral), response.clone()));

            state.mistral_history.push(MistralMessage {
                role: "assistant".to_string(),
                content: response,
            });
        }
        None => (),
    }
    Task::none()
}
