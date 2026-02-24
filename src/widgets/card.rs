//! aOS Card widget — a surface container with header, content, and footer.

use iced::widget::{column, container, text, Space};
use iced::{Element, Length, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled card container.
pub fn aos_card<'a, Message: 'a>(
    title: Option<&'a str>,
    description: Option<&'a str>,
    body: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_LG),
            width: 1.0,
            color: AosTheme::BORDER,
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..container::Style::default()
    };

    let mut col = column![].spacing(AosTheme::SPACING_SM);

    if title.is_some() || description.is_some() {
        let mut header = column![].spacing(AosTheme::SPACING_XS);
        if let Some(t) = title {
            header = header.push(text(t).size(AosTheme::FONT_LG).color(AosTheme::FG_PRIMARY));
        }
        if let Some(d) = description {
            header = header.push(text(d).size(AosTheme::FONT_SM).color(AosTheme::FG_SECONDARY));
        }
        col = col.push(header);
        col = col.push(Space::with_height(AosTheme::SPACING_XS));
    }

    col = col.push(body);

    if let Some(f) = footer {
        col = col.push(Space::with_height(AosTheme::SPACING_SM));
        col = col.push(f);
    }

    container(col)
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_LG))
        .style(style)
        .into()
}
