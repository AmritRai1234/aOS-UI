//! aOS Timeline widget — vertical sequence of events.

use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// A timeline event item.
pub struct TimelineEvent {
    pub title: String,
    pub description: String,
    pub timestamp: String,
}

/// An aOS-styled vertical timeline.
pub fn aos_timeline<'a, Message: 'a>(events: &[TimelineEvent]) -> Element<'a, Message> {
    let mut col = column![].spacing(0.0);

    for (i, event) in events.iter().enumerate() {
        let is_last = i == events.len() - 1;

        let dot_style = move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(AosTheme::ACCENT)),
            border: iced::Border {
                radius: Radius::new(AosTheme::RADIUS_FULL),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            ..container::Style::default()
        };

        let dot = container(Space::new(0, 0))
            .width(Length::Fixed(10.0))
            .height(Length::Fixed(10.0))
            .style(dot_style);

        let content_col = column![
            text(event.title.clone())
                .size(AosTheme::FONT_SM)
                .color(AosTheme::FG_PRIMARY),
            text(event.description.clone())
                .size(AosTheme::FONT_XS)
                .color(AosTheme::FG_SECONDARY),
            text(event.timestamp.clone())
                .size(AosTheme::FONT_XS)
                .color(AosTheme::FG_MUTED),
        ]
        .spacing(AosTheme::SPACING_XS);

        let indicator: Element<'a, Message> = if !is_last {
            let line_style = move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(AosTheme::BORDER)),
                ..container::Style::default()
            };
            container(Space::new(0, 0))
                .width(Length::Fixed(2.0))
                .height(Length::Fixed(40.0))
                .style(line_style)
                .into()
        } else {
            Space::new(0, 0).into()
        };

        let event_row = row![
            column![dot, indicator]
                .align_x(iced::alignment::Horizontal::Center)
                .width(Length::Fixed(20.0)),
            content_col,
        ]
        .spacing(AosTheme::SPACING_MD);

        col = col.push(
            container(event_row).padding(Padding::from([AosTheme::SPACING_XS, 0.0])),
        );
    }

    col.into()
}
