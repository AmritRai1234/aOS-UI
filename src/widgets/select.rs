//! aOS Select (Pick List) widget.

use iced::widget::pick_list;
use iced::Element;
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled select / pick list dropdown.
pub fn aos_select<'a, Message: Clone + 'a, T>(
    options: &'a [T],
    selected: Option<T>,
    on_select: impl Fn(T) -> Message + 'a,
) -> Element<'a, Message>
where
    T: ToString + PartialEq + Clone + 'a,
{
    let style = move |_theme: &iced::Theme, _status: pick_list::Status| pick_list::Style {
        text_color: AosTheme::FG_PRIMARY,
        placeholder_color: AosTheme::FG_MUTED,
        handle_color: AosTheme::FG_SECONDARY,
        background: iced::Background::Color(AosTheme::BG_SURFACE),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_MD),
            width: 1.0,
            color: AosTheme::BORDER,
        },
    };

    pick_list(options, selected, on_select)
        .text_size(AosTheme::FONT_SM)
        .padding(AosTheme::SPACING_SM)
        .style(style)
        .into()
}
