//! aOS Stat Card widget — displays a metric with label and trend.

use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// Trend direction for the stat.
#[derive(Debug, Clone, Copy)]
pub enum Trend {
    Up(f32),
    Down(f32),
    Neutral,
}

/// An aOS-styled stat card.
pub fn aos_stat_card<'a, Message: 'a>(
    label: &'a str,
    value: &'a str,
    trend: Trend,
) -> Element<'a, Message> {
    let card_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_LG),
            width: 1.0,
            color: AosTheme::BORDER,
        },
        ..container::Style::default()
    };

    let (trend_icon, trend_text, trend_color) = match trend {
        Trend::Up(pct) => ("↑", format!("+{:.1}%", pct), AosTheme::SUCCESS),
        Trend::Down(pct) => ("↓", format!("-{:.1}%", pct), AosTheme::ERROR),
        Trend::Neutral => ("→", "0.0%".to_string(), AosTheme::FG_MUTED),
    };

    container(
        column![
            text(label)
                .size(AosTheme::FONT_SM)
                .color(AosTheme::FG_SECONDARY),
            Space::with_height(AosTheme::SPACING_XS),
            row![
                text(value)
                    .size(AosTheme::FONT_2XL)
                    .color(AosTheme::FG_PRIMARY),
                Space::with_width(Length::Fill),
                text(format!("{} {}", trend_icon, trend_text))
                    .size(AosTheme::FONT_SM)
                    .color(trend_color),
            ]
            .align_y(iced::alignment::Vertical::Bottom),
        ]
    )
    .width(Length::Fill)
    .padding(Padding::new(AosTheme::SPACING_LG))
    .style(card_style)
    .into()
}
