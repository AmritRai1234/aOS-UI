//! aOS Toggle Switch widget.

use iced::widget::toggler;
use iced::Element;

use crate::theme::AosTheme;

/// An aOS-styled toggle switch.
pub fn aos_switch<'a, Message: Clone + 'a>(
    label: Option<&'a str>,
    is_toggled: bool,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme, status: toggler::Status| {
        let (bg, dot) = match status {
            toggler::Status::Active { is_toggled } => {
                if is_toggled {
                    (AosTheme::ACCENT, AosTheme::FG_PRIMARY)
                } else {
                    (AosTheme::BG_ELEVATED, AosTheme::FG_MUTED)
                }
            }
            toggler::Status::Hovered { is_toggled } => {
                if is_toggled {
                    (AosTheme::ACCENT_HOVER, AosTheme::FG_PRIMARY)
                } else {
                    (AosTheme::BORDER, AosTheme::FG_SECONDARY)
                }
            }
            toggler::Status::Disabled => {
                (AosTheme::BG_SURFACE, AosTheme::FG_MUTED)
            }
        };

        toggler::Style {
            background: bg,
            background_border_width: 0.0,
            background_border_color: iced::Color::TRANSPARENT,
            foreground: dot,
            foreground_border_width: 0.0,
            foreground_border_color: iced::Color::TRANSPARENT,
        }
    };

    let mut t = toggler(is_toggled)
        .on_toggle(on_toggle)
        .size(22.0)
        .style(style);

    if let Some(l) = label {
        t = t.label(l).text_size(AosTheme::FONT_SM).spacing(AosTheme::SPACING_SM);
    }

    t.into()
}
