//! aOS Skeleton widget — a placeholder loading indicator.

use iced::widget::container;
use iced::{Element, Length};
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled skeleton placeholder.
pub fn aos_skeleton<'a, Message: 'a>(width: Length, height: f32) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_ELEVATED)),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_MD),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        ..container::Style::default()
    };

    container(iced::widget::Space::new(width, Length::Fixed(height)))
        .width(width)
        .height(Length::Fixed(height))
        .style(style)
        .into()
}
