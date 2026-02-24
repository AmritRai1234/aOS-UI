//! aOS Testimonial Card widget — user quote with rating and avatar.

use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;
use crate::widgets::avatar::aos_avatar;

/// An aOS-styled testimonial card.
pub fn aos_testimonial_card<'a, Message: 'a>(
    name: &'a str,
    role: Option<&'a str>,
    company: Option<&'a str>,
    quote: &'a str,
    rating: u8,
) -> Element<'a, Message> {
    let card_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_LG),
            width: 1.0,
            color: AosTheme::BORDER,
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..container::Style::default()
    };

    let mut content = column![].spacing(AosTheme::SPACING_MD);

    let stars: String = (0..5)
        .map(|i| if i < rating { '★' } else { '☆' })
        .collect();
    content = content.push(
        text(stars).size(AosTheme::FONT_LG).color(AosTheme::ACCENT),
    );

    content = content.push(
        text(format!("\"{}\"", quote))
            .size(AosTheme::FONT_SM)
            .color(AosTheme::FG_SECONDARY),
    );

    content = content.push(Space::with_height(AosTheme::SPACING_SM));

    let mut author_info = column![
        text(name)
            .size(AosTheme::FONT_SM)
            .color(AosTheme::FG_PRIMARY),
    ]
    .spacing(AosTheme::SPACING_XS);

    if let Some(r) = role {
        let subtitle = match company {
            Some(c) => format!("{} at {}", r, c),
            None => r.to_string(),
        };
        author_info = author_info.push(
            text(subtitle)
                .size(AosTheme::FONT_XS)
                .color(AosTheme::FG_MUTED),
        );
    }

    content = content.push(
        row![aos_avatar(name, 36.0), author_info]
            .spacing(AosTheme::SPACING_MD)
            .align_y(iced::alignment::Vertical::Center),
    );

    container(content)
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_XL))
        .style(card_style)
        .into()
}
