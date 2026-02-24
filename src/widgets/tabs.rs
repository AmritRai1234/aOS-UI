//! aOS Tabs widget — tab header row + content panels.

use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// A single tab definition.
pub struct Tab<'a, Message> {
    pub label: String,
    pub content: Element<'a, Message>,
}

/// An aOS-styled tab bar with content switching.
pub fn aos_tabs<'a, Message: Clone + 'a>(
    tabs: Vec<Tab<'a, Message>>,
    active: usize,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message> {
    let mut tab_row = row![].spacing(2.0);

    for (i, tab) in tabs.iter().enumerate() {
        let is_active = i == active;
        let label = tab.label.clone();
        let idx = i;

        let tab_style = move |_theme: &iced::Theme, _status: button::Status| {
            let (bg, fg) = if is_active {
                (AosTheme::BG_ELEVATED, AosTheme::FG_PRIMARY)
            } else {
                (iced::Color::TRANSPARENT, AosTheme::FG_MUTED)
            };

            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: fg,
                border: iced::Border {
                    radius: Radius::new(AosTheme::RADIUS_MD),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow: iced::Shadow::default(),
            }
        };

        let msg = on_select(idx);

        tab_row = tab_row.push(
            button(text(label).size(AosTheme::FONT_SM))
                .padding(Padding::from([AosTheme::SPACING_SM, AosTheme::SPACING_LG]))
                .on_press(msg)
                .style(tab_style),
        );
    }

    let tab_bar_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
        ..container::Style::default()
    };

    let tab_bar = container(tab_row).width(Length::Fill).style(tab_bar_style);

    let content_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_MD),
            width: 1.0,
            color: AosTheme::BORDER,
        },
        ..container::Style::default()
    };

    let mut content_elements: Vec<Element<'a, Message>> = tabs
        .into_iter()
        .enumerate()
        .filter_map(|(i, tab)| if i == active { Some(tab.content) } else { None })
        .collect();

    let active_content = if !content_elements.is_empty() {
        content_elements.remove(0)
    } else {
        text("No tab selected").into()
    };

    let content_area = container(active_content)
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_LG))
        .style(content_style);

    column![tab_bar, content_area].into()
}
