//! aOS UI — Premium Widget Showcase
//!
//! A polished design system demo with Inter font, rich visual hierarchy,
//! and premium dark-mode styling.
//!
//! Run with: `cargo run --example showcase`

use iced::widget::{
    column, container, horizontal_space, row, scrollable, text, vertical_space, Space,
};
use iced::{Element, Font, Length, Padding, Task};
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;

use aos_ui::theme::{AosTheme, INTER, INTER_BYTES};
use aos_ui::widgets::badge::{aos_badge, BadgeVariant};
use aos_ui::widgets::button::{aos_button, ButtonVariant};
use aos_ui::widgets::checkbox::aos_checkbox;
use aos_ui::widgets::progress::aos_progress;
use aos_ui::widgets::separator::{aos_separator, Orientation};
use aos_ui::widgets::skeleton::aos_skeleton;
use aos_ui::widgets::slider::aos_slider;
use aos_ui::widgets::spinner::aos_spinner;
use aos_ui::widgets::stat_card::{aos_stat_card, Trend};
use aos_ui::widgets::stepper::aos_stepper;
use aos_ui::widgets::switch::aos_switch;
use aos_ui::widgets::text_input::aos_text_input;
use aos_ui::widgets::avatar::aos_avatar;
use aos_ui::widgets::testimonial_card::aos_testimonial_card;
use aos_ui::widgets::window_controls::aos_window_controls;

fn main() -> iced::Result {
    iced::application("aOS UI — Design System", Showcase::update, Showcase::view)
        .window_size((1200.0, 900.0))
        .font(INTER_BYTES)
        .default_font(INTER)
        .antialiasing(true)
        .run_with(|| (Showcase::default(), Task::none()))
}

#[derive(Default)]
struct Showcase {
    text_value: String,
    checkbox_val: bool,
    switch_val: bool,
    slider_val: f32,
}

#[derive(Debug, Clone)]
enum Msg {
    TextChanged(String),
    CheckboxToggled(bool),
    SwitchToggled(bool),
    SliderChanged(f32),
    ButtonPressed,
    WindowAction(aos_ui::widgets::window_controls::WindowAction),
}

impl Showcase {
    fn update(&mut self, message: Msg) -> Task<Msg> {
        match message {
            Msg::TextChanged(v) => self.text_value = v,
            Msg::CheckboxToggled(v) => self.checkbox_val = v,
            Msg::SwitchToggled(v) => self.switch_val = v,
            Msg::SliderChanged(v) => self.slider_val = v,
            Msg::ButtonPressed => {}
            Msg::WindowAction(_) => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Msg> {
        // ─── Style helpers ──────────────────────────────────────
        let page_bg = move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(AosTheme::BG_BASE)),
            ..container::Style::default()
        };

        let section_card = move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(AosTheme::BG_SURFACE)),
            border: iced::Border {
                radius: Radius::new(AosTheme::RADIUS_LG),
                width: 1.0,
                color: AosTheme::BORDER,
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
            ..container::Style::default()
        };

        let label_pill = move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(AosTheme::ACCENT_SUBTLE)),
            border: iced::Border {
                radius: Radius::new(AosTheme::RADIUS_FULL),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            ..container::Style::default()
        };

        // ─── Helper macros ──────────────────────────────────────
        macro_rules! section_label {
            ($t:expr) => {
                container(
                    text($t)
                        .size(AosTheme::FONT_2XS)
                        .color(AosTheme::ACCENT)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..INTER
                        })
                )
                .padding(Padding::from([3.0, 10.0]))
                .style(label_pill)
            };
        }

        macro_rules! section_title {
            ($t:expr) => {
                text($t)
                    .size(AosTheme::FONT_XL)
                    .color(AosTheme::FG_PRIMARY)
                    .font(Font {
                        weight: iced::font::Weight::Semibold,
                        ..INTER
                    })
            };
        }

        macro_rules! section_desc {
            ($t:expr) => {
                text($t)
                    .size(AosTheme::FONT_SM)
                    .color(AosTheme::FG_MUTED)
            };
        }

        // ─── Title bar ──────────────────────────────────────────
        let title_bar = container(
            row![
                aos_window_controls(Msg::WindowAction),
                horizontal_space(),
                text("aOS Design System")
                    .size(AosTheme::FONT_SM)
                    .color(AosTheme::FG_MUTED)
                    .font(Font {
                        weight: iced::font::Weight::Medium,
                        ..INTER
                    }),
                horizontal_space(),
                // Balance the controls on the right
                Space::with_width(60.0),
            ]
            .align_y(Vertical::Center)
            .padding(Padding::from([AosTheme::SPACING_SM, AosTheme::SPACING_LG])),
        )
        .width(Length::Fill);

        // ─── Hero section ───────────────────────────────────────
        let hero = container(
            column![
                text("aOS")
                    .size(AosTheme::FONT_HERO)
                    .color(AosTheme::FG_PRIMARY)
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..INTER
                    }),
                row![
                    text("UI")
                        .size(AosTheme::FONT_HERO)
                        .color(AosTheme::ACCENT),
                ]
                .spacing(0.0),
                vertical_space().height(AosTheme::SPACING_SM),
                text("A native Iced widget toolkit for the aOS desktop environment.\n26 components · Dark mode · Inter font · GPU accelerated")
                    .size(AosTheme::FONT_BASE)
                    .color(AosTheme::FG_SECONDARY),
                vertical_space().height(AosTheme::SPACING_LG),
                row![
                    aos_badge("v0.1.0", BadgeVariant::Default),
                    aos_badge("Iced 0.13", BadgeVariant::Secondary),
                    aos_badge("Rust", BadgeVariant::Outline),
                ]
                .spacing(AosTheme::SPACING_SM),
            ]
            .align_x(Horizontal::Center)
            .spacing(0.0),
        )
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_3XL))
        .align_x(Horizontal::Center);

        // ─── Buttons section ────────────────────────────────────
        let buttons_section = container(
            column![
                section_label!("COMPONENTS"),
                vertical_space().height(AosTheme::SPACING_SM),
                section_title!("Buttons"),
                section_desc!("Six variant styles for every interaction pattern."),
                vertical_space().height(AosTheme::SPACING_LG),
                row![
                    aos_button("Primary", ButtonVariant::Default, Some(Msg::ButtonPressed)),
                    aos_button("Secondary", ButtonVariant::Secondary, Some(Msg::ButtonPressed)),
                    aos_button("Outline", ButtonVariant::Outline, Some(Msg::ButtonPressed)),
                    aos_button("Ghost", ButtonVariant::Ghost, Some(Msg::ButtonPressed)),
                    aos_button("Destructive", ButtonVariant::Destructive, Some(Msg::ButtonPressed)),
                    aos_button("Link", ButtonVariant::Link, Some(Msg::ButtonPressed)),
                ]
                .spacing(AosTheme::SPACING_SM)
                .align_y(Vertical::Center),
                vertical_space().height(AosTheme::SPACING_LG),
                row![
                    aos_button("Disabled", ButtonVariant::Default, None::<Msg>),
                    aos_button("Disabled", ButtonVariant::Secondary, None::<Msg>),
                    aos_button("Disabled", ButtonVariant::Outline, None::<Msg>),
                ]
                .spacing(AosTheme::SPACING_SM)
                .align_y(Vertical::Center),
            ]
            .spacing(0.0),
        )
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_XL))
        .style(section_card);

        // ─── Inputs section ─────────────────────────────────────
        let inputs_section = container(
            column![
                section_label!("FORM CONTROLS"),
                vertical_space().height(AosTheme::SPACING_SM),
                section_title!("Inputs"),
                section_desc!("Text fields, checkboxes, toggles, and range sliders."),
                vertical_space().height(AosTheme::SPACING_LG),
                aos_text_input("Search components, actions, settings...", &self.text_value, Msg::TextChanged),
                vertical_space().height(AosTheme::SPACING_MD),
                row![
                    column![
                        aos_checkbox("Enable notifications", self.checkbox_val, Msg::CheckboxToggled),
                        vertical_space().height(AosTheme::SPACING_SM),
                        aos_checkbox("Auto-save workspace", true, |_| Msg::CheckboxToggled(true)),
                    ]
                    .width(Length::FillPortion(1)),
                    column![
                        aos_switch(Some("Dark mode"), self.switch_val, Msg::SwitchToggled),
                        vertical_space().height(AosTheme::SPACING_SM),
                        aos_switch(Some("GPU acceleration"), true, |_| Msg::SwitchToggled(true)),
                    ]
                    .width(Length::FillPortion(1)),
                ]
                .spacing(AosTheme::SPACING_XL),
                vertical_space().height(AosTheme::SPACING_LG),
                column![
                    row![
                        text("Volume").size(AosTheme::FONT_SM).color(AosTheme::FG_SECONDARY),
                        horizontal_space(),
                        text(format!("{}%", self.slider_val as u32)).size(AosTheme::FONT_SM).color(AosTheme::ACCENT),
                    ],
                    vertical_space().height(AosTheme::SPACING_XS),
                    aos_slider(0.0..=100.0, self.slider_val, Msg::SliderChanged),
                ],
            ]
            .spacing(0.0),
        )
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_XL))
        .style(section_card);

        // ─── Indicators section ─────────────────────────────────
        let indicators_section = container(
            column![
                section_label!("FEEDBACK"),
                vertical_space().height(AosTheme::SPACING_SM),
                section_title!("Status Indicators"),
                section_desc!("Badges, progress bars, avatars, and loading states."),
                vertical_space().height(AosTheme::SPACING_LG),
                // Badges row
                row![
                    aos_badge("Default", BadgeVariant::Default),
                    aos_badge("Success", BadgeVariant::Success),
                    aos_badge("Warning", BadgeVariant::Warning),
                    aos_badge("Error", BadgeVariant::Error),
                    aos_badge("Secondary", BadgeVariant::Secondary),
                    aos_badge("Outline", BadgeVariant::Outline),
                ]
                .spacing(AosTheme::SPACING_SM)
                .align_y(Vertical::Center),
                vertical_space().height(AosTheme::SPACING_LG),
                // Progress
                column![
                    row![
                        text("System load").size(AosTheme::FONT_SM).color(AosTheme::FG_SECONDARY),
                        horizontal_space(),
                        text("65%").size(AosTheme::FONT_SM).color(AosTheme::ACCENT),
                    ],
                    vertical_space().height(AosTheme::SPACING_XS),
                    aos_progress(0.0..=100.0, 65.0),
                ],
                vertical_space().height(AosTheme::SPACING_SM),
                column![
                    row![
                        text("Memory usage").size(AosTheme::FONT_SM).color(AosTheme::FG_SECONDARY),
                        horizontal_space(),
                        text("42%").size(AosTheme::FONT_SM).color(AosTheme::SUCCESS),
                    ],
                    vertical_space().height(AosTheme::SPACING_XS),
                    aos_progress(0.0..=100.0, 42.0),
                ],
                vertical_space().height(AosTheme::SPACING_LG),
                // Avatars + loading
                row![
                    aos_avatar("Amrit Rai", 44.0),
                    aos_avatar("OpenBSD", 44.0),
                    aos_avatar("Jane D", 44.0),
                    Space::with_width(AosTheme::SPACING_LG),
                    text("│").size(AosTheme::FONT_LG).color(AosTheme::BORDER),
                    Space::with_width(AosTheme::SPACING_LG),
                    aos_spinner(),
                    Space::with_width(AosTheme::SPACING_LG),
                    aos_skeleton(Length::Fixed(100.0), 18.0),
                    aos_skeleton(Length::Fixed(60.0), 18.0),
                ]
                .spacing(AosTheme::SPACING_SM)
                .align_y(Vertical::Center),
            ]
            .spacing(0.0),
        )
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_XL))
        .style(section_card);

        // ─── Stats section ──────────────────────────────────────
        let stats_section = column![
            container(
                column![
                    section_label!("ANALYTICS"),
                    vertical_space().height(AosTheme::SPACING_SM),
                    section_title!("Dashboard Stats"),
                    section_desc!("Real-time metric cards with trend indicators."),
                ]
                .spacing(0.0),
            )
            .padding(Padding::from([0.0, AosTheme::SPACING_XS])),
            vertical_space().height(AosTheme::SPACING_LG),
            row![
                aos_stat_card("Active Users", "14,892", Trend::Up(12.5)),
                aos_stat_card("Revenue", "$48.2K", Trend::Up(8.1)),
                aos_stat_card("Bounce Rate", "23.4%", Trend::Down(3.2)),
            ]
            .spacing(AosTheme::SPACING_MD),
        ]
        .spacing(0.0);

        // ─── Stepper section ────────────────────────────────────
        let stepper_section = container(
            column![
                section_label!("NAVIGATION"),
                vertical_space().height(AosTheme::SPACING_SM),
                section_title!("Stepper"),
                section_desc!("Multi-step wizard progress indicator."),
                vertical_space().height(AosTheme::SPACING_XL),
                aos_stepper(&["Account", "Profile", "Preferences", "Review", "Complete"], 2),
            ]
            .spacing(0.0),
        )
        .width(Length::Fill)
        .padding(Padding::new(AosTheme::SPACING_XL))
        .style(section_card);

        // ─── Testimonials section ───────────────────────────────
        let testimonials_section = column![
            container(
                column![
                    section_label!("SOCIAL PROOF"),
                    vertical_space().height(AosTheme::SPACING_SM),
                    section_title!("Testimonials"),
                    section_desc!("User quote cards with ratings and avatars."),
                ],
            )
            .padding(Padding::from([0.0, AosTheme::SPACING_XS])),
            vertical_space().height(AosTheme::SPACING_LG),
            row![
                aos_testimonial_card(
                    "Amrit Rai",
                    Some("Founder"),
                    Some("aOS"),
                    "Building a native desktop shell with these components is incredibly fast. The Elm Architecture keeps everything predictable.",
                    5,
                ),
                aos_testimonial_card(
                    "Jane Smith",
                    Some("Engineer"),
                    Some("OpenBSD"),
                    "Clean, performant, and beautifully designed. Finally a native widget toolkit that doesn't look stuck in 2005.",
                    4,
                ),
            ]
            .spacing(AosTheme::SPACING_MD),
        ]
        .spacing(0.0);

        // ─── Footer ─────────────────────────────────────────────
        let footer = container(
            column![
                aos_separator(Orientation::Horizontal),
                vertical_space().height(AosTheme::SPACING_LG),
                row![
                    text("aOS-UI v0.1.0")
                        .size(AosTheme::FONT_XS)
                        .color(AosTheme::FG_MUTED),
                    horizontal_space(),
                    text("Built with Iced · Rust · ♥")
                        .size(AosTheme::FONT_XS)
                        .color(AosTheme::FG_MUTED),
                ]
                .align_y(Vertical::Center),
                vertical_space().height(AosTheme::SPACING_LG),
            ]
        )
        .width(Length::Fill);

        // ─── Page assembly ──────────────────────────────────────
        let page = column![
            title_bar,
            hero,
            buttons_section,
            vertical_space().height(AosTheme::SPACING_XL),
            inputs_section,
            vertical_space().height(AosTheme::SPACING_XL),
            indicators_section,
            vertical_space().height(AosTheme::SPACING_XL),
            stats_section,
            vertical_space().height(AosTheme::SPACING_XL),
            stepper_section,
            vertical_space().height(AosTheme::SPACING_XL),
            testimonials_section,
            vertical_space().height(AosTheme::SPACING_3XL),
            footer,
        ]
        .padding(Padding::from([0.0, AosTheme::SPACING_3XL]))
        .max_width(960);

        // Centered scrollable page on dark background
        container(
            scrollable(
                container(page)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                    .padding(Padding::from([AosTheme::SPACING_XS, 0.0])),
            )
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(page_bg)
        .into()
    }
}
