use iced::{
    border::Radius,
    widget::{button, container},
    Background, Border, Color,
};

pub const BLUE_SKY: [f32; 3] = [0.8, 0.9, 1.0];
pub const GRAY: [f32; 3] = [0.9, 0.9, 0.9];
pub const AI_LABEL_COLOR: [f32; 3] = [1.0, 0.0, 0.0];

pub fn card(color: [f32; 3]) -> container::Style {
    container::Style {
        background: Some(Background::Color(color.into())),
        border: Border {
            radius: Radius::from(10.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn primary_button(status: button::Status) -> button::Style {
    let (background, text_color, border_color) = match status {
        button::Status::Hovered => ([0.2, 0.6, 1.0], [1.0, 1.0, 1.0], [0.1, 0.5, 0.9]),
        button::Status::Pressed => ([0.1, 0.5, 0.9], [1.0, 1.0, 1.0], [0.0, 0.4, 0.8]),
        _ => ([0.3, 0.7, 1.0], [1.0, 1.0, 1.0], [0.2, 0.6, 1.0]),
    };

    button::Style {
        background: Some(Background::Color(background.into())),
        text_color: Color::from_rgb(text_color[0], text_color[1], text_color[2]),
        border: Border {
            color: Color::from_rgb(border_color[0], border_color[1], border_color[2]),
            width: 2.0,
            radius: Radius::from(8.0), // Rounded corners
        },
        ..Default::default()
    }
}

pub fn danger_button(status: button::Status) -> button::Style {
    let (background, text_color, border_color) = match status {
        button::Status::Hovered => ([1.0, 0.4, 0.4], [1.0, 1.0, 1.0], [0.9, 0.3, 0.3]),
        button::Status::Pressed => ([0.9, 0.3, 0.3], [1.0, 1.0, 1.0], [0.8, 0.2, 0.2]),
        _ => ([1.0, 0.5, 0.5], [1.0, 1.0, 1.0], [0.9, 0.4, 0.4]),
    };

    button::Style {
        background: Some(Background::Color(background.into())),
        text_color: Color::from_rgb(text_color[0], text_color[1], text_color[2]),
        border: Border {
            color: Color::from_rgb(border_color[0], border_color[1], border_color[2]),
            width: 2.0,
            radius: Radius::from(8.0), // Rounded corners
        },
        ..Default::default()
    }
}
