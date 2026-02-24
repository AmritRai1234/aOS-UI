//! # aOS-UI
//!
//! A native Iced widget toolkit for the aOS desktop environment.
//! Provides a comprehensive set of UI components translated from
//! a React/TypeScript design system into idiomatic Rust.

pub mod theme;
pub mod widgets;

pub use theme::AosTheme;
pub use widgets::*;
