//! aOS Checkbox widget.

use iced::widget::checkbox;
use iced::Element;
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled checkbox.
pub fn aos_checkbox<'a, Message: Clone + 'a>(
    label: &'a str,
    is_checked: bool,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme, status: checkbox::Status| {
        let (bg, border, icon_color) = match status {
            checkbox::Status::Active { is_checked } => {
                if is_checked {
                    (AosTheme::ACCENT, AosTheme::ACCENT, AosTheme::FG_PRIMARY)
                } else {
                    (AosTheme::BG_SURFACE, AosTheme::BORDER, AosTheme::FG_MUTED)
                }
            }
            checkbox::Status::Hovered { is_checked } => {
                if is_checked {
                    (AosTheme::ACCENT_HOVER, AosTheme::ACCENT_HOVER, AosTheme::FG_PRIMARY)
                } else {
                    (AosTheme::BG_ELEVATED, AosTheme::BORDER_ACTIVE, AosTheme::FG_MUTED)
                }
            }
            checkbox::Status::Disabled { .. } => {
                (AosTheme::BG_SURFACE, AosTheme::BORDER, AosTheme::FG_MUTED)
            }
        };

        checkbox::Style {
            background: iced::Background::Color(bg),
            icon_color,
            border: iced::Border {
                radius: Radius::new(AosTheme::RADIUS_SM),
                width: 1.5,
                color: border,
            },
            text_color: Some(AosTheme::FG_PRIMARY),
        }
    };

    checkbox(label, is_checked)
        .on_toggle(on_toggle)
        .size(18.0)
        .text_size(AosTheme::FONT_SM)
        .spacing(AosTheme::SPACING_SM)
        .style(style)
        .into()
}
