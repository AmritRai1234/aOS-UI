//! aOS Radio button widget.

use iced::widget::radio;
use iced::Element;

use crate::theme::AosTheme;

/// An aOS-styled radio button.
pub fn aos_radio<'a, Message: Clone + 'a, V: Copy + Eq + 'a>(
    label: &'a str,
    value: V,
    selected: Option<V>,
    on_click: impl Fn(V) -> Message + 'a,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme, status: radio::Status| {
        let (dot_color, bg, border) = match status {
            radio::Status::Active { is_selected } => {
                if is_selected {
                    (AosTheme::ACCENT, AosTheme::BG_SURFACE, AosTheme::ACCENT)
                } else {
                    (iced::Color::TRANSPARENT, AosTheme::BG_SURFACE, AosTheme::BORDER)
                }
            }
            radio::Status::Hovered { is_selected } => {
                if is_selected {
                    (AosTheme::ACCENT_HOVER, AosTheme::BG_ELEVATED, AosTheme::ACCENT_HOVER)
                } else {
                    (iced::Color::TRANSPARENT, AosTheme::BG_ELEVATED, AosTheme::BORDER_ACTIVE)
                }
            }
        };

        radio::Style {
            background: iced::Background::Color(bg),
            dot_color,
            border_width: 1.5,
            border_color: border,
            text_color: Some(AosTheme::FG_PRIMARY),
        }
    };

    radio(label, value, selected, on_click)
        .size(18.0)
        .text_size(AosTheme::FONT_SM)
        .spacing(AosTheme::SPACING_SM)
        .style(style)
        .into()
}
