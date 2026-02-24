//! aOS Stepper widget — multi-step progress indicator with labels.

use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length, Padding};
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled step indicator.
pub fn aos_stepper<'a, Message: 'a>(
    steps: &[&'a str],
    current_step: usize,
) -> Element<'a, Message> {
    let mut step_row = row![].spacing(0.0).align_y(Vertical::Center);

    for (i, label) in steps.iter().enumerate() {
        let is_done = i < current_step;
        let is_current = i == current_step;

        let (circle_bg, circle_fg) = if is_done {
            (AosTheme::SUCCESS, AosTheme::FG_PRIMARY)
        } else if is_current {
            (AosTheme::ACCENT, AosTheme::FG_PRIMARY)
        } else {
            (AosTheme::BG_ELEVATED, AosTheme::FG_MUTED)
        };

        let circle_style = move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(circle_bg)),
            border: iced::Border {
                radius: Radius::new(AosTheme::RADIUS_FULL),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            ..container::Style::default()
        };

        let step_num = if is_done {
            "✓".to_string()
        } else {
            format!("{}", i + 1)
        };

        let circle = container(
            text(step_num)
                .size(AosTheme::FONT_XS)
                .color(circle_fg)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
        )
        .width(Length::Fixed(28.0))
        .height(Length::Fixed(28.0))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(circle_style);

        let label_color = if is_done || is_current {
            AosTheme::FG_PRIMARY
        } else {
            AosTheme::FG_MUTED
        };

        let step_col = column![
            circle,
            text(*label).size(AosTheme::FONT_XS).color(label_color),
        ]
        .spacing(AosTheme::SPACING_XS)
        .align_x(Horizontal::Center);

        step_row = step_row.push(step_col);

        if i < steps.len() - 1 {
            let line_color = if is_done {
                AosTheme::SUCCESS
            } else {
                AosTheme::BORDER
            };

            let line_style = move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(line_color)),
                ..container::Style::default()
            };

            step_row = step_row.push(
                container(Space::new(0, 0))
                    .width(Length::Fixed(40.0))
                    .height(Length::Fixed(2.0))
                    .style(line_style),
            );
        }
    }

    container(step_row)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .padding(Padding::new(AosTheme::SPACING_MD))
        .into()
}
