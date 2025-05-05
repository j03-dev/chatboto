use iced::{
    border::Radius,
    widget::{column, container, overlay::menu, pick_list, row, text_editor},
    Background, Color, Element, Length, Task,
};

use rusql_alchemy::prelude::*;

use crate::{
    components::{button, message_area, nav_bar, text_input::text_area},
    models::Config,
    services,
    styles::{self, BLUE_SKY},
    types::{AIMessage, Gam, MessageType, Version},
    AIChoice, Message, State,
};

pub fn chat(state: &State) -> Element<Message> {
    let choices = [
        AIChoice::Gemini(Version::V1_5, Gam::Flash),
        AIChoice::Gemini(Version::V1_5, Gam::Pro),
        AIChoice::Gemini(Version::V2_0, Gam::Flash),
        AIChoice::Gemini(Version::V2_0, Gam::Pro),
        AIChoice::Gemini(Version::V2_5, Gam::Flash),
        AIChoice::Gemini(Version::V2_5, Gam::Pro),
        AIChoice::Mistral,
    ];
    column![
        nav_bar::nav_bar(),
        message_area::chat_area(state.messages.clone()),
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
            button::rounded_button("Send", Message::Submit, |_, status| styles::primary_button(
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
    let conn = state.conn.clone();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        if let Ok(Some(config)) = Config::get(kwargs!(id == 1), &conn).await {
            Config {
                ai_choice: Some(choice.to_string()),
                ..config
            }
            .update(&conn)
            .await
            .ok();
        }
    });
    Task::none()
}

pub fn action_submit(state: &mut State) -> Task<Message> {
    let value = state.content.text();

    if value.trim().is_empty() {
        return Task::none();
    }

    state.messages.push((MessageType::Sent, value.clone()));

    let api_key = match state.ai_choice {
        Some(AIChoice::Gemini(_, _)) => state.forms.get("gemini").cloned().unwrap_or_default(),
        Some(AIChoice::Mistral) => state.forms.get("mistral").cloned().unwrap_or_default(),
        None => "".to_string(),
    };
    let choice = state.ai_choice.unwrap_or_default();
    let history = match choice {
        AIChoice::Gemini(_, _) => state.gemini_history.clone(),
        AIChoice::Mistral => state.mistral_history.clone(),
    };
    let task = Task::perform(services::ask_ai(choice, value, history, api_key), |resp| {
        Message::AIRespond(resp.unwrap_or_else(|err| err.to_string()))
    });

    state.content = text_editor::Content::new();
    task
}

pub fn handle_ai_response(state: &mut State, response: String) -> Task<Message> {
    match state.ai_choice {
        Some(AIChoice::Gemini(_, _)) => {
            state.messages.push((
                MessageType::Received(AIChoice::Gemini(Version::default(), Gam::default())),
                response.clone(),
            ));

            state.gemini_history.push(AIMessage {
                role: "model".to_string(),
                content: response,
            });
        }
        Some(AIChoice::Mistral) => {
            state
                .messages
                .push((MessageType::Received(AIChoice::Mistral), response.clone()));

            state.mistral_history.push(AIMessage {
                role: "assistant".to_string(),
                content: response,
            });
        }
        None => (),
    }
    Task::none()
}
