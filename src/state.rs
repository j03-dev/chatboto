use iced::time::Duration;
use iced::widget::text_editor;
use rusql_alchemy::prelude::*;

use crate::{
    models::Config,
    types::{AIChoice, AIMessage, FormState, Gam, MessageType, Screen, Version},
};

pub struct State {
    pub messages: Vec<(MessageType, String)>,
    pub ai_choice: Option<AIChoice>,
    pub gemini_history: Vec<AIMessage>,
    pub mistral_history: Vec<AIMessage>,
    pub content: text_editor::Content,
    pub screen: Screen,
    pub forms: FormState,
    pub conn: Connection,

    pub tick: Duration,
    pub timer_enabled: bool,

    pub message: String,
}

impl Default for State {
    fn default() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let (conn, config) = runtime.block_on(async {
            let database = Database::new().await.unwrap();
            database.migrate().await.ok();
            let conn = database.conn;
            let config = Config::get(kwargs!(id == 1), &conn).await.unwrap();

            if config.is_none() {
                Config::default().save(&conn).await.unwrap();
            }

            (conn, config)
        });

        let forms = config
            .as_ref()
            .map(|cfg| {
                FormState::from([
                    (
                        "mistral".to_string(),
                        cfg.mistral_apikey.clone().unwrap_or_default(),
                    ),
                    (
                        "gemini".to_string(),
                        cfg.gemini_apikey.clone().unwrap_or_default(),
                    ),
                ])
            })
            .unwrap_or_default();

        let ai_choice = config
            .as_ref()
            .and_then(|cfg| {
                cfg.ai_choice.as_ref().map(|choice| match choice.as_str() {
                    "mistral" => AIChoice::Mistral,
                    "gemini-1.5-flash" => AIChoice::Gemini(Version::V1_5, Gam::Flash),
                    "gemini-1.5-pro" => AIChoice::Gemini(Version::V1_5, Gam::Pro),
                    "gemini-2.0-flash" => AIChoice::Gemini(Version::V2_0, Gam::Flash),
                    "gemini-2.0-pro" => AIChoice::Gemini(Version::V2_0, Gam::Pro),
                    "gemini-2.5-flash" => AIChoice::Gemini(Version::V2_5, Gam::Flash),
                    "gemini-2.5-pro" => AIChoice::Gemini(Version::V2_5, Gam::Pro),
                    _ => panic!("ai choice should 'gemini' or 'mistral'"),
                })
            })
            .unwrap_or_default();

        Self {
            messages: Vec::new(),
            ai_choice: Some(ai_choice),
            gemini_history: Vec::new(),
            mistral_history: Vec::new(),
            content: text_editor::Content::new(),
            screen: Screen::ChatScreen,
            conn,
            forms,
            timer_enabled: false,
            tick: Duration::default(),
            message: String::new(),
        }
    }
}
