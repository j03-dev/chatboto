use iced::{
    widget::{row, Space},
    Element, Length,
};

use crate::{styles, Message, Screen};

use super::button::rounded_button;

pub fn nav_bar<'l>() -> Element<'l, Message> {
    row![
        Space::with_width(Length::Fill),
        rounded_button(
            "setting",
            Message::Change(Screen::SettingScreen),
            |_, status| { styles::primary_button(status) }
        )
    ]
    .into()
}
pub mod actions {
    use iced::Task;

    use crate::{Message, Screen, State};

    pub fn change(state: &mut State, screen: Screen) -> Task<Message> {
        state.screen = screen;
        Task::none()
    }
}
