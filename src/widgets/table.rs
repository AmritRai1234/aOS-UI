//! aOS Table widget — header + data rows.

use iced::widget::{column, container, row, text, scrollable};
use iced::{Element, Length, Padding};
use iced::border::Radius;

use crate::theme::AosTheme;

/// An aOS-styled data table.
pub fn aos_table<'a, Message: 'a>(
    headers: &[String],
    rows: &[Vec<String>],
) -> Element<'a, Message> {
    let header_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_ELEVATED)),
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_MD),
            width: 1.0,
            color: AosTheme::BORDER,
        },
        ..container::Style::default()
    };

    let row_style = move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
        border: iced::Border {
            radius: Radius::new(0.0),
            width: 0.0,
            color: AosTheme::BORDER,
        },
        ..container::Style::default()
    };

    let mut header_row = row![].spacing(0.0);
    for h in headers {
        header_row = header_row.push(
            container(
                text(h.clone())
                    .size(AosTheme::FONT_XS)
                    .color(AosTheme::FG_MUTED),
            )
            .width(Length::Fill)
            .padding(Padding::from([AosTheme::SPACING_SM, AosTheme::SPACING_MD])),
        );
    }
    let header = container(header_row)
        .width(Length::Fill)
        .style(header_style);

    let mut body = column![].spacing(0.0);
    for cells in rows {
        let mut data_row = row![].spacing(0.0);
        for cell in cells {
            data_row = data_row.push(
                container(
                    text(cell.clone())
                        .size(AosTheme::FONT_SM)
                        .color(AosTheme::FG_PRIMARY),
                )
                .width(Length::Fill)
                .padding(Padding::from([AosTheme::SPACING_SM, AosTheme::SPACING_MD])),
            );
        }
        body = body.push(container(data_row).width(Length::Fill).style(row_style));
    }

    let outer_style = move |_theme: &iced::Theme| container::Style {
        border: iced::Border {
            radius: Radius::new(AosTheme::RADIUS_MD),
            width: 1.0,
            color: AosTheme::BORDER,
        },
        ..container::Style::default()
    };

    container(
        column![header, scrollable(body).height(Length::Shrink)]
    )
    .width(Length::Fill)
    .style(outer_style)
    .into()
}
