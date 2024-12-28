use iced::{
    border::Radius,
    widget::{button, container},
    Background, Border, Color,
};

pub const BLUE_SKY: [f32; 3] = [0.8, 0.9, 1.0];
pub const GRAY: [f32; 3] = [0.9, 0.9, 0.9];

pub fn card(color: [f32; 3]) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(color.into())),
        border: Border {
            radius: Radius::from(10.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn menu_button(status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color([0.9, 0.9, 0.9].into())), // Lighter gray
            text_color: Color::from_rgb(0.1, 0.1, 0.1),                  // Darker text
            border: Border {
                color: Color::from_rgb(0.5, 0.5, 0.5), // Gray border
                width: 2.0,                            // Thickness of bottom border
                radius: Radius::from(0.0),             // No rounding
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color([0.7, 0.7, 0.7].into())), // Darker gray
            text_color: Color::WHITE,                                    // White text when pressed
            border: Border {
                color: Color::from_rgb(0.5, 0.5, 0.5), // Gray border
                width: 2.0,                            // Thickness of bottom border
                radius: Radius::from(0.0),             // No rounding
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(Background::Color([0.8, 0.8, 0.8].into())), // Default gray
            text_color: Color::from_rgb(0.2, 0.2, 0.2),                  // Dark gray text
            border: Border {
                color: Color::from_rgb(0.5, 0.5, 0.5), // Gray border
                width: 2.0,                            // Thickness of bottom border
                radius: Radius::from(0.0),             // No rounding
            },
            ..Default::default()
        },
    }
}

/// Defines the style for a cancel menu button based on its status.
pub fn cancel_menu_button(status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color([0.95, 0.7, 0.7].into())), // Soft red background
            text_color: Color::from_rgb(0.2, 0.2, 0.2),                   // Dark gray text
            border: Border {
                color: Color::from_rgb(0.7, 0.5, 0.5), // Soft red border
                width: 2.0,                            // Thickness of bottom border
                radius: Radius::from(0.0),             // No rounding
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color([0.85, 0.5, 0.5].into())), // Slightly darker red background
            text_color: Color::WHITE,                                     // White text
            border: Border {
                color: Color::from_rgb(0.6, 0.4, 0.4), // Darker red border
                width: 2.0,                            // Thickness of bottom border
                radius: Radius::from(0.0),             // No rounding
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(Background::Color([0.9, 0.6, 0.6].into())), // Default soft red background
            text_color: Color::from_rgb(0.2, 0.2, 0.2),                  // Dark gray text
            border: Border {
                color: Color::from_rgb(0.7, 0.4, 0.4), // Soft red border
                width: 2.0,                            // Thickness of bottom border
                radius: Radius::from(0.0),             // No rounding
            },
            ..Default::default()
        },
    }
}
