use iced::{
    border::Radius,
    widget::{column, container, overlay::menu, pick_list, row},
    Background, Color, Element, Length,
};

use crate::{
    commons::types::Agent,
    components::{button, forms, message_area, nav_bar},
    models::{
        gemini::*,
        mistral::{MistralLargeLatest, MistralModel},
    },
    styles::{self, BLUE_SKY},
    Message, State,
};

pub fn chat_screen(state: &'_ State) -> Element<'_, Message> {
    let agents = [
        Agent::Gemini(GeminiModel::GeminiV15Flash(GeminiV15Flash::default())),
        Agent::Gemini(GeminiModel::GeminiV15Pro(GeminiV15Pro::default())),
        Agent::Gemini(GeminiModel::GeminiV20Flash(GeminiV20Flash::default())),
        Agent::Gemini(GeminiModel::GeminiV20Pro(GeminiV20Pro::default())),
        Agent::Gemini(GeminiModel::GeminiV25Flash(GeminiV25Flash::default())),
        Agent::Gemini(GeminiModel::GeminiV25Pro(GeminiV25Pro::default())),
        Agent::Mistral(MistralModel::MistralLargeLatest(
            MistralLargeLatest::default(),
        )),
    ];
    column![
        nav_bar::nav_bar(),
        message_area::chat_area(state.messages.clone()),
        row![
            container(forms::text_area::form(&state.content)).max_height(200),
            pick_list(agents, state.agent.clone(), Message::Selected)
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

pub mod actions {
    use iced::{widget::text_editor, Task};

    use crate::{
        commons::types::{AIMessage, Agent, Role, TypeAgent},
        schemas::Config,
        Message, State,
    };
    use rusql_alchemy::prelude::*;

    pub fn select(state: &mut State, agent: &Agent) -> Task<Message> {
        state.agent = Some(agent.clone());
        let conn = state.conn.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            if let Ok(Some(config)) = Config::get(kwargs!(id == 1), &conn).await {
                Config {
                    ai_choice: Some(agent.to_string()),
                    ..config
                }
                .update(&conn)
                .await
                .ok();
            }
        });
        Task::none()
    }

    pub fn submit(state: &mut State) -> Task<Message> {
        let value = state.content.text();

        if value.trim().is_empty() {
            return Task::none();
        }

        state.messages.push((Role::User, value.clone()));

        let api_key = match state.agent {
            Some(Agent::Gemini(_)) => state.forms.get("gemini").cloned().unwrap_or_default(),
            Some(Agent::Mistral(_)) => state.forms.get("mistral").cloned().unwrap_or_default(),
            None => {
                return Task::done(Message::AIRespond(
                    "Api key is not present so pls add your api key on setting.".to_string(),
                ));
            }
        };

        let agent = state.agent.clone().unwrap_or_default();

        let history = match agent {
            Agent::Gemini(_) => state.gemini_history.clone(),
            Agent::Mistral(_) => state.mistral_history.clone(),
        };

        let task = match agent {
            Agent::Gemini(model) => model.ask(api_key, value, history),
            Agent::Mistral(model) => model.ask(api_key, value, history),
        };

        state.content = text_editor::Content::new();

        task
    }

    pub fn push_response(state: &mut State, response: String) -> Task<Message> {
        match state.agent {
            Some(Agent::Gemini(_)) => {
                state
                    .messages
                    .push((Role::Agent(TypeAgent::Gemini), response.clone()));

                state.gemini_history.push(AIMessage {
                    role: "model".to_string(),
                    content: response,
                });
            }
            Some(Agent::Mistral(_)) => {
                state
                    .messages
                    .push((Role::Agent(TypeAgent::Mistral), response.clone()));

                state.mistral_history.push(AIMessage {
                    role: "assistant".to_string(),
                    content: response,
                });
            }
            None => (),
        }
        Task::none()
    }
}
