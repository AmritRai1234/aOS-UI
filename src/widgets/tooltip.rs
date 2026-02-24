//! aOS Tooltip widget — styled wrapper around Iced's tooltip.

use iced::widget::{container, text, tooltip};
use iced::{Element, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// Tooltip position.
pub use tooltip::Position as TooltipPosition;

/// An aOS-styled tooltip wrapping any element.
pub fn aos_tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tip: &'a str,
    position: TooltipPosition,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_ELEVATED)),
        text_color: Some(AosTheme::FG_PRIMARY),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_SM),
            width: 1.0,
            color: AosTheme::BORDER,
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
    };

    tooltip(
        content,
        container(text(tip).size(AosTheme::FONT_XS))
            .padding(Padding::from([4.0, 8.0]))
            .style(style),
        position,
    )
    .gap(4.0)
    .into()
}
