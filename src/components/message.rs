use iced::{widget::text, Element, Length};

use crate::{styles, Message};

#[derive(Default, Debug, Clone)]
pub enum Status {
    Fail = 0,
    #[default]
    Sucess,
}

pub fn message<'l>(message: &'l str, status: &Status) -> Element<'l, Message> {
    text(message)
        .size(20)
        .color(styles::message(status))
        .center()
        .width(Length::Fill)
        .into()
}

pub mod actions {
    use std::time::Duration;

    use iced::Task;

    use crate::{Message, State};

    use super::Status;

    pub fn trigger(msg: &str, status: Status, duration: u64) -> Task<Message> {
        Task::done(Message::DisplayMessage {
            duration: Duration::from_secs(duration),
            msg: msg.to_owned(),
            status,
        })
    }

    pub fn display(
        state: &mut State,
        message: String,
        status: Status,
        duration: Duration,
    ) -> Task<Message> {
        state.timer_enabled = true;
        state.tick = duration;
        state.message = message;
        state.message_status = status;
        Task::none()
    }

    pub fn update_timer(state: &mut State) -> Task<Message> {
        if state.tick > Duration::default() {
            state.tick -= Duration::from_secs(1);
        } else {
            state.timer_enabled = false;
            state.message = String::new();
            state.tick = Duration::default();
        }
        Task::none()
    }
}
