//! aOS Pricing Table widget — tier comparison cards.

use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;
use crate::widgets::button::{aos_button, ButtonVariant};
use crate::widgets::badge::{aos_badge, BadgeVariant};

/// A pricing tier definition.
pub struct PricingTier<Message> {
    pub name: String,
    pub description: String,
    pub price: String,
    pub period: String,
    pub features: Vec<String>,
    pub is_popular: bool,
    pub cta_label: String,
    pub on_select: Message,
}

/// An aOS-styled pricing table.
pub fn aos_pricing_table<'a, Message: Clone + 'a>(
    tiers: Vec<PricingTier<Message>>,
) -> Element<'a, Message> {
    let mut tier_row = row![].spacing(AosTheme::SPACING_LG);

    for tier in tiers {
        let is_popular = tier.is_popular;
        let border_color = if is_popular {
            AosTheme::ACCENT
        } else {
            AosTheme::BORDER
        };

        let card_style = move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
            border: iced::Border {
                radius: Radius::new(AosTheme::RADIUS_LG),
                width: if is_popular { 2.0 } else { 1.0 },
                color: border_color,
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..container::Style::default()
        };

        let mut content = column![].spacing(AosTheme::SPACING_MD);

        if is_popular {
            content = content.push(aos_badge("POPULAR", BadgeVariant::Default));
        }

        // Use owned Strings via text() which takes Into<String>
        content = content.push(
            text(tier.name.clone())
                .size(AosTheme::FONT_XL)
                .color(AosTheme::FG_PRIMARY),
        );
        content = content.push(
            text(tier.description.clone())
                .size(AosTheme::FONT_SM)
                .color(AosTheme::FG_SECONDARY),
        );

        content = content.push(
            row![
                text(tier.price.clone())
                    .size(AosTheme::FONT_3XL)
                    .color(AosTheme::FG_PRIMARY),
                text(format!("/{}", tier.period))
                    .size(AosTheme::FONT_SM)
                    .color(AosTheme::FG_MUTED),
            ]
            .spacing(AosTheme::SPACING_XS)
            .align_y(iced::alignment::Vertical::Bottom),
        );

        for feature in &tier.features {
            content = content.push(
                row![
                    text("✓").size(AosTheme::FONT_SM).color(AosTheme::SUCCESS),
                    text(feature.clone())
                        .size(AosTheme::FONT_SM)
                        .color(AosTheme::FG_SECONDARY),
                ]
                .spacing(AosTheme::SPACING_SM),
            );
        }

        content = content.push(Space::with_height(AosTheme::SPACING_SM));

        let variant = if is_popular {
            ButtonVariant::Default
        } else {
            ButtonVariant::Outline
        };
        let cta = tier.cta_label.clone();
        content = content.push(aos_button(
            Box::leak(cta.into_boxed_str()),
            variant,
            Some(tier.on_select),
        ));

        tier_row = tier_row.push(
            container(content)
                .width(Length::Fill)
                .padding(Padding::new(AosTheme::SPACING_XL))
                .style(card_style),
        );
    }

    tier_row.into()
}
