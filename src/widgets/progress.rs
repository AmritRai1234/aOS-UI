//! aOS Progress Bar widget.

use iced::widget::progress_bar;
use iced::Element;
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled progress bar.
pub fn aos_progress<'a, Message: 'a>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme| progress_bar::Style {
        background: iced::Background::Color(AosTheme::BG_ELEVATED),
        bar: iced::Background::Color(AosTheme::ACCENT),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_FULL),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
    };

    progress_bar(range, value)
        .height(6.0)
        .style(style)
        .into()
}
