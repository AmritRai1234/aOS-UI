//! aOS Design System — Colors, spacing, typography, and radius tokens.
//!
//! This module defines the visual identity of aOS: a deep-space dark theme
//! with vibrant accent hues, inspired by modern desktop environments.

use iced::Color;
use iced::Font;

// ─── Custom Font ─────────────────────────────────────────────
pub const INTER: Font = Font::with_name("Inter");
pub const INTER_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter.ttf");

// ─── Color Palette ───────────────────────────────────────────

/// Core color tokens for the aOS theme.
pub struct AosTheme;

impl AosTheme {
    // ── Background (deep space blacks with subtle blue undertone) ──
    pub const BG_BASE: Color = Color::from_rgb(0.055, 0.059, 0.082);     // #0e0f15
    pub const BG_SURFACE: Color = Color::from_rgb(0.082, 0.090, 0.125);  // #151720
    pub const BG_ELEVATED: Color = Color::from_rgb(0.110, 0.118, 0.165); // #1c1e2a
    pub const BG_CARD: Color = Color::from_rgb(0.094, 0.102, 0.145);     // #181a25
    pub const BG_OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.05, 0.7);
    pub const BG_HOVER: Color = Color::from_rgb(0.133, 0.141, 0.196);    // #222432

    // ── Foreground ──
    pub const FG_PRIMARY: Color = Color::from_rgb(0.93, 0.94, 0.97);     // #edf0f8
    pub const FG_SECONDARY: Color = Color::from_rgb(0.58, 0.60, 0.70);   // #959ab3
    pub const FG_MUTED: Color = Color::from_rgb(0.35, 0.37, 0.47);       // #595e78

    // ── Accent (electric indigo → violet) ──
    pub const ACCENT: Color = Color::from_rgb(0.38, 0.42, 1.0);          // #616bff
    pub const ACCENT_HOVER: Color = Color::from_rgb(0.48, 0.52, 1.0);    // #7a85ff
    pub const ACCENT_SUBTLE: Color = Color::from_rgba(0.38, 0.42, 1.0, 0.12);
    pub const ACCENT_GLOW: Color = Color::from_rgba(0.38, 0.42, 1.0, 0.25);

    // ── Semantic ──
    pub const SUCCESS: Color = Color::from_rgb(0.16, 0.82, 0.55);        // #29d18c
    pub const SUCCESS_SUBTLE: Color = Color::from_rgba(0.16, 0.82, 0.55, 0.12);
    pub const WARNING: Color = Color::from_rgb(1.0, 0.68, 0.16);         // #ffad29
    pub const WARNING_SUBTLE: Color = Color::from_rgba(1.0, 0.68, 0.16, 0.12);
    pub const ERROR: Color = Color::from_rgb(0.96, 0.26, 0.34);          // #f54257
    pub const ERROR_SUBTLE: Color = Color::from_rgba(0.96, 0.26, 0.34, 0.12);
    pub const INFO: Color = Color::from_rgb(0.22, 0.65, 1.0);            // #38a6ff

    // ── Border ──
    pub const BORDER: Color = Color::from_rgb(0.16, 0.17, 0.24);         // #292b3d
    pub const BORDER_SUBTLE: Color = Color::from_rgb(0.12, 0.13, 0.18);  // #1f212e
    pub const BORDER_ACTIVE: Color = Color::from_rgb(0.38, 0.42, 1.0);   // #616bff

    // ── Spacing (in logical pixels) ──
    pub const SPACING_2XS: f32 = 2.0;
    pub const SPACING_XS: f32 = 4.0;
    pub const SPACING_SM: f32 = 8.0;
    pub const SPACING_MD: f32 = 12.0;
    pub const SPACING_LG: f32 = 16.0;
    pub const SPACING_XL: f32 = 24.0;
    pub const SPACING_2XL: f32 = 32.0;
    pub const SPACING_3XL: f32 = 48.0;
    pub const SPACING_4XL: f32 = 64.0;

    // ── Border Radius ──
    pub const RADIUS_SM: f32 = 6.0;
    pub const RADIUS_MD: f32 = 10.0;
    pub const RADIUS_LG: f32 = 14.0;
    pub const RADIUS_XL: f32 = 20.0;
    pub const RADIUS_FULL: f32 = 9999.0;

    // ── Font Sizes ──
    pub const FONT_2XS: f32 = 10.0;
    pub const FONT_XS: f32 = 11.0;
    pub const FONT_SM: f32 = 13.0;
    pub const FONT_BASE: f32 = 15.0;
    pub const FONT_LG: f32 = 18.0;
    pub const FONT_XL: f32 = 24.0;
    pub const FONT_2XL: f32 = 32.0;
    pub const FONT_3XL: f32 = 40.0;
    pub const FONT_HERO: f32 = 56.0;
}
