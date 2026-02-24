//! aOS Badge widget — a small label with colored background.

use iced::widget::{container, text};
use iced::{Element, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// Badge variant styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Success,
    Warning,
    Error,
    Outline,
}

/// An aOS-styled badge.
pub fn aos_badge<'a, Message: 'a>(label: &'a str, variant: BadgeVariant) -> Element<'a, Message> {
    let (bg, fg, border_c) = match variant {
        BadgeVariant::Default => (AosTheme::ACCENT, AosTheme::FG_PRIMARY, AosTheme::ACCENT),
        BadgeVariant::Secondary => (AosTheme::BG_ELEVATED, AosTheme::FG_SECONDARY, AosTheme::BORDER),
        BadgeVariant::Success => (AosTheme::SUCCESS, AosTheme::FG_PRIMARY, AosTheme::SUCCESS),
        BadgeVariant::Warning => (AosTheme::WARNING, AosTheme::BG_BASE, AosTheme::WARNING),
        BadgeVariant::Error => (AosTheme::ERROR, AosTheme::FG_PRIMARY, AosTheme::ERROR),
        BadgeVariant::Outline => {
            (iced::Color::TRANSPARENT, AosTheme::FG_PRIMARY, AosTheme::BORDER)
        }
    };

    let style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(bg)),
        text_color: Some(fg),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_FULL),
            width: if matches!(variant, BadgeVariant::Outline) { 1.0 } else { 0.0 },
            color: border_c,
        },
        shadow: iced::Shadow::default(),
    };

    container(text(label).size(AosTheme::FONT_XS))
        .padding(Padding::from([2.0, AosTheme::SPACING_SM]))
        .style(style)
        .into()
}
