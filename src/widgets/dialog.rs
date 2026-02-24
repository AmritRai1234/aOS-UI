//! aOS Dialog widget — modal overlay container.

use iced::widget::{column, container, mouse_area, text, Space, stack};
use iced::{Element, Length, Padding};
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled modal dialog overlay.
pub fn aos_dialog<'a, Message: Clone + 'a>(
    title: &'a str,
    description: Option<&'a str>,
    body: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    on_dismiss: Message,
    base: Element<'a, Message>,
) -> Element<'a, Message> {
    let backdrop_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_OVERLAY)),
        ..container::Style::default()
    };

    let card_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_XL),
            width: 1.0,
            color: AosTheme::BORDER,
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.4),
            offset: iced::Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..container::Style::default()
    };

    let mut dialog_content = column![].spacing(AosTheme::SPACING_MD);

    dialog_content = dialog_content.push(
        text(title).size(AosTheme::FONT_XL).color(AosTheme::FG_PRIMARY),
    );

    if let Some(desc) = description {
        dialog_content = dialog_content.push(
            text(desc).size(AosTheme::FONT_SM).color(AosTheme::FG_SECONDARY),
        );
    }

    dialog_content = dialog_content.push(body);

    if let Some(f) = footer {
        dialog_content = dialog_content.push(Space::with_height(AosTheme::SPACING_SM));
        dialog_content = dialog_content.push(f);
    }

    let dialog_card = container(dialog_content)
        .width(Length::Fixed(480.0))
        .padding(Padding::new(AosTheme::SPACING_XL))
        .style(card_style);

    let overlay = mouse_area(
        container(dialog_card)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(backdrop_style),
    )
    .on_press(on_dismiss);

    stack![base, overlay].into()
}
