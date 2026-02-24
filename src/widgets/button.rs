//! aOS Button widget — styled wrapper around Iced's button.

use iced::widget::{button, text};
use iced::{Element, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// Button variant styles matching the React design system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

/// An aOS-styled button.
pub fn aos_button<'a, Message: Clone + 'a>(
    label: &'a str,
    variant: ButtonVariant,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme, status: button::Status| {
        let (bg, fg, border_color) = match variant {
            ButtonVariant::Default => (AosTheme::ACCENT, AosTheme::FG_PRIMARY, AosTheme::ACCENT),
            ButtonVariant::Secondary => {
                (AosTheme::BG_ELEVATED, AosTheme::FG_PRIMARY, AosTheme::BORDER)
            }
            ButtonVariant::Outline => {
                (iced::Color::TRANSPARENT, AosTheme::FG_PRIMARY, AosTheme::BORDER)
            }
            ButtonVariant::Ghost => {
                (iced::Color::TRANSPARENT, AosTheme::FG_SECONDARY, iced::Color::TRANSPARENT)
            }
            ButtonVariant::Destructive => (AosTheme::ERROR, AosTheme::FG_PRIMARY, AosTheme::ERROR),
            ButtonVariant::Link => {
                (iced::Color::TRANSPARENT, AosTheme::ACCENT, iced::Color::TRANSPARENT)
            }
        };

        let bg = match status {
            button::Status::Hovered => iced::Background::Color(iced::Color {
                a: bg.a.max(0.1) + 0.08,
                ..bg
            }),
            button::Status::Pressed => iced::Background::Color(iced::Color {
                a: bg.a.max(0.1) - 0.05,
                ..bg
            }),
            _ => iced::Background::Color(bg),
        };

        button::Style {
            background: Some(bg),
            text_color: fg,
            border: iced::Border {
                color: border_color,
                width: if matches!(variant, ButtonVariant::Outline) {
                    1.0
                } else {
                    0.0
                },
                radius: Radius::new(AosTheme::RADIUS_MD),
            },
            shadow: iced::Shadow::default(),
        }
    };

    let btn = button(
        text(label).size(AosTheme::FONT_SM),
    )
    .padding(Padding::from([AosTheme::SPACING_SM, AosTheme::SPACING_LG]))
    .style(style);

    let btn = if let Some(msg) = on_press {
        btn.on_press(msg)
    } else {
        btn
    };

    btn.into()
}
