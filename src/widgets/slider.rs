//! aOS Slider widget.

use iced::widget::slider;
use iced::Element;
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled slider.
pub fn aos_slider<'a, Message: Clone + 'a>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme, status: slider::Status| {
        let handle_color = match status {
            slider::Status::Hovered => AosTheme::ACCENT_HOVER,
            slider::Status::Dragged => AosTheme::ACCENT,
            _ => AosTheme::ACCENT,
        };

        slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    iced::Background::Color(AosTheme::ACCENT),
                    iced::Background::Color(AosTheme::BG_ELEVATED),
                ),
                width: 4.0,
                border: iced::Border {
                    radius: Radius::new(AosTheme::RADIUS_FULL),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 8.0 },
                background: iced::Background::Color(handle_color),
                border_width: 2.0,
                border_color: AosTheme::FG_PRIMARY,
            },
        }
    };

    slider(range, value, on_change)
        .step(0.01)
        .style(style)
        .into()
}
