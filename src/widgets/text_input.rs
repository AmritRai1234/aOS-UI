//! aOS Text Input widget — styled wrapper around Iced's text_input.

use iced::widget::text_input;
use iced::Element;
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled text input field.
pub fn aos_text_input<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme, status: text_input::Status| {
        let border_color = match status {
            text_input::Status::Focused => AosTheme::BORDER_ACTIVE,
            _ => AosTheme::BORDER,
        };

        text_input::Style {
            background: iced::Background::Color(AosTheme::BG_SURFACE),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: Radius::new(AosTheme::RADIUS_MD),
            },
            icon: AosTheme::FG_MUTED,
            placeholder: AosTheme::FG_MUTED,
            value: AosTheme::FG_PRIMARY,
            selection: AosTheme::ACCENT_SUBTLE,
        }
    };

    text_input(placeholder, value)
        .on_input(on_input)
        .size(AosTheme::FONT_BASE)
        .padding(AosTheme::SPACING_SM)
        .style(style)
        .into()
}
