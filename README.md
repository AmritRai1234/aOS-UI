# aOS-UI

A native **Iced** (Rust) widget toolkit for the aOS desktop environment — a custom OpenBSD fork with a modern, GPU-accelerated UI.

## Overview

This library translates a comprehensive React/TypeScript component library into idiomatic Rust using the [Iced](https://iced.rs) GUI framework. Every widget follows The Elm Architecture (Model → Message → Update → View) and renders natively via `wgpu`.

## Widgets

### Primitives
`button` · `text_input` · `checkbox` · `radio` · `switch` · `slider` · `progress` · `badge` · `avatar` · `separator` · `skeleton` · `spinner` · `tooltip`

### Layout
`card` · `tabs` · `accordion` · `dialog` · `select` · `table`

### Composite
`navbar` · `stat_card` · `timeline` · `stepper` · `window_controls` · `pricing_table` · `testimonial_card`

## Quick Start

```bash
# Build the library
cargo build

# Run the widget showcase
cargo run --example showcase
```

## Usage

```rust
use aos_ui::widgets::button::{aos_button, ButtonVariant};
use aos_ui::widgets::card::aos_card;
use aos_ui::theme::AosTheme;

// Create a styled button
let btn = aos_button("Click Me", ButtonVariant::Default, Some(MyMessage::Click));

// Create a card
let card = aos_card(
    Some("Title"),
    Some("Description"),
    my_content,
    None,
);
```

## License

MIT
