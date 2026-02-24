//! aOS Navbar widget — top navigation bar with title and actions.

use iced::widget::{button, container, row, text, Space};
use iced::{Element, Length, Padding};
use iced::alignment::Vertical;
use iced::border::Radius;

use crate::theme::AosTheme;

/// A navbar action item.
pub struct NavAction<Message> {
    pub label: String,
    pub message: Message,
}

/// An aOS-styled top navigation bar.
pub fn aos_navbar<'a, Message: Clone + 'a>(
    title: &'a str,
    actions: Vec<NavAction<Message>>,
) -> Element<'a, Message> {
    let bar_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
        border: iced::Border {
            radius: Radius::new(0.0),
            width: 0.0,
            color: AosTheme::BORDER,
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 4.0,
        },
        ..container::Style::default()
    };

    let mut action_row = row![].spacing(AosTheme::SPACING_SM).align_y(Vertical::Center);
    for action in actions {
        let nav_btn_style = move |_theme: &iced::Theme, _status: button::Status| button::Style {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            text_color: AosTheme::FG_SECONDARY,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        };

        action_row = action_row.push(
            button(text(action.label).size(AosTheme::FONT_SM))
                .padding(Padding::from([AosTheme::SPACING_XS, AosTheme::SPACING_SM]))
                .on_press(action.message)
                .style(nav_btn_style),
        );
    }

    container(
        row![
            text(title)
                .size(AosTheme::FONT_LG)
                .color(AosTheme::FG_PRIMARY),
            Space::with_width(Length::Fill),
            action_row,
        ]
        .align_y(Vertical::Center)
        .padding(Padding::from([AosTheme::SPACING_SM, AosTheme::SPACING_LG])),
    )
    .width(Length::Fill)
    .style(bar_style)
    .into()
}
