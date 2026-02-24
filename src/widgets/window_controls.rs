//! aOS Window Controls widget — macOS-style traffic light buttons.

use iced::widget::{button, row, Space};
use iced::{Element, Length};
use iced::border::Radius;

use crate::theme::AosTheme;

/// Window control action.
#[derive(Debug, Clone)]
pub enum WindowAction {
    Close,
    Minimize,
    Maximize,
}

/// An aOS-styled window control buttons (traffic light).
pub fn aos_window_controls<'a, Message: Clone + 'a>(
    on_action: impl Fn(WindowAction) -> Message + 'a,
) -> Element<'a, Message> {
    let close_style = move |_theme: &iced::Theme, status: button::Status| {
        let bg = match status {
            button::Status::Hovered => iced::Color::from_rgb(1.0, 0.35, 0.35),
            _ => iced::Color::from_rgb(0.94, 0.27, 0.27),
        };
        circle_btn_style(bg)
    };

    let minimize_style = move |_theme: &iced::Theme, status: button::Status| {
        let bg = match status {
            button::Status::Hovered => iced::Color::from_rgb(1.0, 0.78, 0.25),
            _ => iced::Color::from_rgb(0.98, 0.72, 0.15),
        };
        circle_btn_style(bg)
    };

    let maximize_style = move |_theme: &iced::Theme, status: button::Status| {
        let bg = match status {
            button::Status::Hovered => iced::Color::from_rgb(0.35, 0.85, 0.35),
            _ => iced::Color::from_rgb(0.18, 0.78, 0.24),
        };
        circle_btn_style(bg)
    };

    row![
        button(Space::new(0, 0))
            .width(Length::Fixed(12.0))
            .height(Length::Fixed(12.0))
            .on_press(on_action(WindowAction::Close))
            .style(close_style),
        button(Space::new(0, 0))
            .width(Length::Fixed(12.0))
            .height(Length::Fixed(12.0))
            .on_press(on_action(WindowAction::Minimize))
            .style(minimize_style),
        button(Space::new(0, 0))
            .width(Length::Fixed(12.0))
            .height(Length::Fixed(12.0))
            .on_press(on_action(WindowAction::Maximize))
            .style(maximize_style),
    ]
    .spacing(AosTheme::SPACING_SM)
    .into()
}

fn circle_btn_style(bg: iced::Color) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(bg)),
        text_color: iced::Color::TRANSPARENT,
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_FULL),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        shadow: iced::Shadow::default(),
    }
}
