//! aOS Design System — Colors, spacing, typography, and radius tokens.
//!
//! This module defines the visual identity of aOS, inspired by modern
//! dark-mode desktop environments with vibrant accent colors.

use iced::Color;

// ─── Color Palette ───────────────────────────────────────────────

/// Core color tokens for the aOS theme.
pub struct AosTheme;

impl AosTheme {
    // ── Background ──
    pub const BG_BASE: Color = Color::from_rgb(0.07, 0.07, 0.10);        // #121219
    pub const BG_SURFACE: Color = Color::from_rgb(0.10, 0.10, 0.14);     // #1a1a24
    pub const BG_ELEVATED: Color = Color::from_rgb(0.14, 0.14, 0.19);    // #242430
    pub const BG_OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.6);

    // ── Foreground ──
    pub const FG_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.97);     // #f2f2f7
    pub const FG_SECONDARY: Color = Color::from_rgb(0.63, 0.63, 0.69);   // #a1a1b0
    pub const FG_MUTED: Color = Color::from_rgb(0.40, 0.40, 0.47);       // #666678

    // ── Accent ──
    pub const ACCENT: Color = Color::from_rgb(0.40, 0.52, 1.0);          // #6684ff
    pub const ACCENT_HOVER: Color = Color::from_rgb(0.47, 0.58, 1.0);    // #7894ff
    pub const ACCENT_SUBTLE: Color = Color::from_rgba(0.40, 0.52, 1.0, 0.15);

    // ── Semantic ──
    pub const SUCCESS: Color = Color::from_rgb(0.20, 0.78, 0.52);        // #34c884
    pub const WARNING: Color = Color::from_rgb(1.0, 0.72, 0.25);         // #ffb840
    pub const ERROR: Color = Color::from_rgb(0.94, 0.30, 0.35);          // #f04d59
    pub const INFO: Color = Color::from_rgb(0.25, 0.68, 1.0);            // #40adff

    // ── Border ──
    pub const BORDER: Color = Color::from_rgb(0.20, 0.20, 0.26);         // #333342
    pub const BORDER_ACTIVE: Color = Color::from_rgb(0.40, 0.52, 1.0);   // #6684ff

    // ── Spacing (in logical pixels) ──
    pub const SPACING_XS: f32 = 4.0;
    pub const SPACING_SM: f32 = 8.0;
    pub const SPACING_MD: f32 = 12.0;
    pub const SPACING_LG: f32 = 16.0;
    pub const SPACING_XL: f32 = 24.0;
    pub const SPACING_2XL: f32 = 32.0;
    pub const SPACING_3XL: f32 = 48.0;

    // ── Border Radius ──
    pub const RADIUS_SM: f32 = 4.0;
    pub const RADIUS_MD: f32 = 8.0;
    pub const RADIUS_LG: f32 = 12.0;
    pub const RADIUS_XL: f32 = 16.0;
    pub const RADIUS_FULL: f32 = 9999.0;

    // ── Font Sizes ──
    pub const FONT_XS: f32 = 11.0;
    pub const FONT_SM: f32 = 13.0;
    pub const FONT_BASE: f32 = 15.0;
    pub const FONT_LG: f32 = 18.0;
    pub const FONT_XL: f32 = 22.0;
    pub const FONT_2XL: f32 = 28.0;
    pub const FONT_3XL: f32 = 36.0;
}
