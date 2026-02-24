//! aOS Separator widget — a thin horizontal or vertical divider.

use iced::widget::container;
use iced::{Element, Length};

use crate::theme::AosTheme;

/// Separator orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// An aOS-styled separator line.
pub fn aos_separator<'a, Message: 'a>(orientation: Orientation) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BORDER)),
        ..container::Style::default()
    };

    match orientation {
        Orientation::Horizontal => container(iced::widget::Space::new(Length::Fill, 0))
            .width(Length::Fill)
            .height(Length::Fixed(1.0))
            .style(style)
            .into(),
        Orientation::Vertical => container(iced::widget::Space::new(0, Length::Fill))
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(style)
            .into(),
    }
}
