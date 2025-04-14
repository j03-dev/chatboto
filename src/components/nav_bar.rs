use iced::{
    widget::{row, Space},
    Element, Length, Task,
};

use crate::{styles, Message, Screen, State};

use super::button::rounded_button;

pub fn nav_bar<'l>() -> Element<'l, Message> {
    row![
        Space::with_width(Length::Fill),
        rounded_button(
            "setting",
            Message::Route(Screen::SettingScreen),
            |_, status| { styles::primary_button(status) }
        )
    ]
    .into()
}

pub fn router_pushed(state: &mut State, screen: Screen) -> Task<Message> {
    state.screen = screen;
    Task::none()
}
