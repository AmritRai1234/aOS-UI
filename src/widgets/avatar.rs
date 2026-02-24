//! aOS Avatar widget — circular container with initials fallback.

use iced::widget::{container, text};
use iced::{Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled avatar showing initials.
pub fn aos_avatar<'a, Message: 'a>(
    name: &str,
    size: f32,
) -> Element<'a, Message> {
    let initials: String = name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let font_size = size * 0.4;

    let style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::ACCENT_SUBTLE)),
        text_color: Some(AosTheme::ACCENT),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_FULL),
            width: 1.5,
            color: AosTheme::BORDER,
        },
        shadow: iced::Shadow::default(),
    };

    container(
        text(initials)
            .size(font_size)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    )
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .style(style)
    .into()
}
