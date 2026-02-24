//! aOS Spinner widget — a simple loading indicator text.

use iced::widget::text;
use iced::Element;

use crate::theme::AosTheme;

/// An aOS-styled loading spinner (text-based).
pub fn aos_spinner<'a, Message: 'a>() -> Element<'a, Message> {
    text("⟳ Loading...")
        .size(AosTheme::FONT_SM)
        .color(AosTheme::FG_MUTED)
        .into()
}
