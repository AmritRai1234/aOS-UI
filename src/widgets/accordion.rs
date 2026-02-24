//! aOS Accordion widget — collapsible sections.

use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// A single accordion item definition.
pub struct AccordionItem<'a, Message> {
    pub title: String,
    pub content: Element<'a, Message>,
}

/// An aOS-styled accordion with collapsible sections.
pub fn aos_accordion<'a, Message: Clone + 'a>(
    items: Vec<AccordionItem<'a, Message>>,
    open_index: Option<usize>,
    on_toggle: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message> {
    let mut col = column![].spacing(1.0);

    for (i, item) in items.into_iter().enumerate() {
        let is_open = open_index == Some(i);
        let arrow = if is_open { "▼" } else { "▶" };

        let header_style = move |_theme: &iced::Theme, _status: button::Status| button::Style {
            background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
            text_color: AosTheme::FG_PRIMARY,
            border: iced::Border {
                radius: Radius::new(AosTheme::RADIUS_MD),
                width: 1.0,
                color: AosTheme::BORDER,
            },
            shadow: iced::Shadow::default(),
        };

        let msg = on_toggle(i);

        let header = button(
            row![
                text(arrow).size(AosTheme::FONT_XS).color(AosTheme::FG_MUTED),
                text(item.title).size(AosTheme::FONT_SM),
            ]
            .spacing(AosTheme::SPACING_SM)
            .align_y(iced::alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_MD))
        .on_press(msg)
        .style(header_style);

        col = col.push(header);

        if is_open {
            let content_style = move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(AosTheme::BG_BASE)),
                border: iced::Border {
                    radius: Radius::new(0.0),
                    width: 1.0,
                    color: AosTheme::BORDER,
                },
                ..container::Style::default()
            };

            col = col.push(
                container(item.content)
                    .width(Length::Fill)
                    .padding(Padding::new(AosTheme::SPACING_LG))
                    .style(content_style),
            );
        }
    }

    col.into()
}
