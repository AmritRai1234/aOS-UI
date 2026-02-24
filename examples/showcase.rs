//! aOS UI — Widget Showcase Example
//!
//! Run with: `cargo run --example showcase`

use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Element, Length, Padding, Task};

use aos_ui::theme::AosTheme;
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
    iced::application("aOS UI Showcase", Showcase::update, Showcase::view)
        .window_size((1100.0, 800.0))
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
    Noop,
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
            Msg::Noop => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<Msg> {
        let bg_style = move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(AosTheme::BG_BASE)),
            ..container::Style::default()
        };

        let section_title = |t: &str| {
            text(t)
                .size(AosTheme::FONT_XL)
                .color(AosTheme::ACCENT)
        };

        let content = column![
            // ── Window Controls ──
            row![
                aos_window_controls(Msg::WindowAction),
                Space::with_width(Length::Fill),
                text("aOS UI Showcase")
                    .size(AosTheme::FONT_SM)
                    .color(AosTheme::FG_MUTED),
            ]
            .align_y(iced::alignment::Vertical::Center),
            Space::with_height(AosTheme::SPACING_XL),

            // ── Buttons ──
            section_title("Buttons"),
            Space::with_height(AosTheme::SPACING_SM),
            row![
                aos_button("Default", ButtonVariant::Default, Some(Msg::ButtonPressed)),
                aos_button("Secondary", ButtonVariant::Secondary, Some(Msg::ButtonPressed)),
                aos_button("Outline", ButtonVariant::Outline, Some(Msg::ButtonPressed)),
                aos_button("Ghost", ButtonVariant::Ghost, Some(Msg::ButtonPressed)),
                aos_button("Destructive", ButtonVariant::Destructive, Some(Msg::ButtonPressed)),
                aos_button("Link", ButtonVariant::Link, Some(Msg::ButtonPressed)),
            ]
            .spacing(AosTheme::SPACING_SM),
            Space::with_height(AosTheme::SPACING_LG),
            aos_separator(Orientation::Horizontal),
            Space::with_height(AosTheme::SPACING_LG),

            // ── Inputs ──
            section_title("Inputs"),
            Space::with_height(AosTheme::SPACING_SM),
            aos_text_input("Type something...", &self.text_value, Msg::TextChanged),
            Space::with_height(AosTheme::SPACING_SM),
            row![
                aos_checkbox("Remember me", self.checkbox_val, Msg::CheckboxToggled),
                Space::with_width(AosTheme::SPACING_XL),
                aos_switch(Some("Dark mode"), self.switch_val, Msg::SwitchToggled),
            ]
            .align_y(iced::alignment::Vertical::Center),
            Space::with_height(AosTheme::SPACING_SM),
            aos_slider(0.0..=100.0, self.slider_val, Msg::SliderChanged),
            Space::with_height(AosTheme::SPACING_LG),
            aos_separator(Orientation::Horizontal),
            Space::with_height(AosTheme::SPACING_LG),

            // ── Indicators ──
            section_title("Indicators"),
            Space::with_height(AosTheme::SPACING_SM),
            aos_progress(0.0..=100.0, 65.0),
            Space::with_height(AosTheme::SPACING_SM),
            row![
                aos_badge("Default", BadgeVariant::Default),
                aos_badge("Success", BadgeVariant::Success),
                aos_badge("Warning", BadgeVariant::Warning),
                aos_badge("Error", BadgeVariant::Error),
                aos_badge("Outline", BadgeVariant::Outline),
            ]
            .spacing(AosTheme::SPACING_SM),
            Space::with_height(AosTheme::SPACING_SM),
            row![
                aos_avatar("Amrit Rai", 40.0),
                aos_avatar("Jane Doe", 40.0),
                aos_avatar("OpenBSD", 40.0),
                aos_spinner(),
                aos_skeleton(Length::Fixed(120.0), 20.0),
            ]
            .spacing(AosTheme::SPACING_SM)
            .align_y(iced::alignment::Vertical::Center),
            Space::with_height(AosTheme::SPACING_LG),
            aos_separator(Orientation::Horizontal),
            Space::with_height(AosTheme::SPACING_LG),

            // ── Stat Cards ──
            section_title("Stat Cards"),
            Space::with_height(AosTheme::SPACING_SM),
            row![
                aos_stat_card("Users", "14,892", Trend::Up(12.5)),
                aos_stat_card("Revenue", "$48.2K", Trend::Up(8.1)),
                aos_stat_card("Bounce Rate", "23.4%", Trend::Down(3.2)),
            ]
            .spacing(AosTheme::SPACING_MD),
            Space::with_height(AosTheme::SPACING_LG),
            aos_separator(Orientation::Horizontal),
            Space::with_height(AosTheme::SPACING_LG),

            // ── Stepper ──
            section_title("Stepper"),
            Space::with_height(AosTheme::SPACING_SM),
            aos_stepper(&["Account", "Profile", "Review", "Complete"], 2),
            Space::with_height(AosTheme::SPACING_LG),
            aos_separator(Orientation::Horizontal),
            Space::with_height(AosTheme::SPACING_LG),

            // ── Testimonial ──
            section_title("Testimonial"),
            Space::with_height(AosTheme::SPACING_SM),
            row![
                aos_testimonial_card(
                    "Amrit Rai",
                    Some("Founder"),
                    Some("aOS"),
                    "This component library made building our OS desktop shell incredibly fast.",
                    5,
                ),
                aos_testimonial_card(
                    "Jane Smith",
                    Some("Engineer"),
                    Some("OpenBSD"),
                    "Clean, performant, and beautifully designed widgets.",
                    4,
                ),
            ]
            .spacing(AosTheme::SPACING_MD),
        ]
        .spacing(0.0)
        .padding(Padding::new(AosTheme::SPACING_XL));

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(bg_style)
            .into()
    }
}
