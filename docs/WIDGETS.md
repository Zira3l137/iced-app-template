# Widget Guide

This document explains the custom widgets and macros available in the `{{project-name}}` template.

## Table of Contents

1. [Overview](#overview)
2. [Builder Pattern vs Macros](#builder-pattern-vs-macros)
3. [Text Widgets](#text-widgets)
4. [Interactive Widgets](#interactive-widgets)
5. [Container Widgets](#container-widgets)
6. [Gradients](#gradients)
7. [Styling Guide](#styling-guide)

---

## Overview

`{{project-name}}` provides several custom widgets and macros to make UI development easier and more consistent:

- **`nerd_text!`** - Text with Nerd Font support (for icons in text)
- **`icon!`** - Standalone icon widget
- **`clickable_text!`** - Hoverable, clickable text (like a hyperlink)
- **`frame!`** - Styled container with borders and shadows
- **`button!`** - Highly customizable button
- **`gradient!`** - Linear gradients for backgrounds

All widgets use the **Monaco Ligaturized Nerd Font** by default, which provides:
- Ligatures for programming
- Nerd Font icons for UI

---

## Builder Pattern vs Macros

`{{project-name}}` offers two approaches for creating styled widgets:

### 1. Macros (Recommended for most cases)

Quick and concise syntax using macros:

```rust
use crate::{icon, button, frame};

let my_icon = icon!(Icon::Home, size: 24, color: Color::WHITE);
let my_button = button!(
    "Click me",
    background_active: Color::BLUE,
    border_radius: 8.0
);
```

### 2. Builder Pattern (For advanced customization)

More flexible, type-safe builders:

```rust
use crate::app::widgets::*;

let my_icon = IconBuilder::new(Icon::Home)
    .size(24)
    .color(Color::WHITE)
    .build();

let my_button = ButtonBuilder::new("Click me")
    .background_active(Color::BLUE)
    .border_radius(8.0)
    .build();
```

**When to use which:**
- **Macros**: Quick widget creation, simple styling
- **Builders**: Complex conditional styling, reusable configurations

---

## Text Widgets

### `nerd_text!` - Text with Icon Support

Display text with the custom font, optionally with size and color.

**Basic Usage:**

```rust
use crate::nerd_text;

// Simple text
nerd_text!("Hello World")

// With size
nerd_text!("Hello World", size = 24.0)

// With color
nerd_text!("Hello World", color = iced::Color::from_rgb(1.0, 0.0, 0.0))

// With both
nerd_text!("Hello World", size = 20.0, color = palette.primary)
```

**With Formatting:**

```rust
let name = "Alice";
let count = 42;

// Format string with arguments
nerd_text!(("Hello, {}!", name))

// Multiple arguments
nerd_text!(("Count: {} items", count), size = 18.0)

// With styling
nerd_text!(
    ("Status: {} - {} items", status, count),
    size = 16.0,
    color = palette.success
)
```

**With Icons:**

```rust
use crate::core::types::Icon;

// Icon in text
nerd_text!(("{} Settings", Icon::Settings))

// Icon with variable
nerd_text!(("{} Count: {}", Icon::Variable, count))
```

---

### `icon!` - Standalone Icon

Display a Nerd Font icon by itself.

**Usage:**

```rust
use crate::{icon, core::types::Icon};

// Basic icon
icon!(Icon::Settings)

// With size
icon!(Icon::Settings, size = 32.0)

// With color
icon!(Icon::Settings, color = palette.primary)

// With both
icon!(Icon::Delete, size = 24.0, color = palette.danger)
```

**Available Icons:**

See `src/core/types.rs` for the full list. Common ones:

```rust
Icon::SettingsAlt, // 
Icon::Duplicate,   // 
Icon::Variable,    // 󱃻
Icon::Settings,    // 
Icon::Delete,      // 󰆴
Icon::Cancel,      // 󰜺
Icon::Import,      // 󰋺
Icon::Export,      // 󰮓
Icon::Apply,       // 
Icon::Theme,       // 
Icon::About,       // 
Icon::Edit,        // 
Icon::Copy,        // 󰆏
// ...
```

---

### `clickable_text!` - Interactive Text

Create text that changes appearance on hover and can be clicked.

**Usage:**

```rust
use crate::clickable_text;

// Basic
clickable_text!("Click me")
    .on_press(Message::TextClicked)

// With colors
clickable_text!(
    "Click me",
    color = palette.primary,
    color_hovered = palette.primary * 0.8
).on_press(Message::TextClicked)

// With formatting
let url = "https://example.com";
clickable_text!(
    ("{}", url),
    size = 14.0,
    color = palette.primary,
    color_hovered = palette_ext.primary.strong.color
).on_press(Message::OpenUrl(url.to_string()))
```

**Example - Hyperlink:**

```rust
let link = clickable_text!(
    constants::APP_REPOSITORY,
    color = palette_ext.primary.base.color,
    color_hovered = palette_ext.primary.strong.color
).on_press(Message::OpenRepo);

// Use in a row
row![
    nerd_text!("Repository: "),
    link
]
```

---

## Interactive Widgets

### `button!` - Customizable Button

Create buttons with full control over appearance across different states.

**Basic Usage:**

```rust
use crate::button;

// Simple button
button!(nerd_text!("Click me"))
    .on_press(Message::ButtonClicked)

// With rounded corners
button!(
    nerd_text!("Save"),
    border_radius: 8.0
).on_press(Message::Save)
```

**Custom Colors:**

```rust
// Success button (green)
button!(
    nerd_text!(("{} Save", Icon::Apply)),
    background_active: palette_ext.success.base.color,
    border_radius: 8.0
).on_press(Message::Save)

// Danger button (red)
button!(
    icon!(Icon::Delete),
    background_active: palette_ext.danger.base.color,
    border_radius: 8.0
).on_press(Message::Delete)

// Warning button (yellow/orange)
button!(
    icon!(Icon::Edit),
    background_active: iced::Color::from_rgb(0.7, 0.6, 0.0),
    border_radius: 8.0
).on_press(Message::Edit)
```

**State-Specific Styling:**

```rust
button!(
    "Hover me",
    background_active: Color::BLUE,
    background_hovered: Color::from_rgb(0.3, 0.7, 1.0),
    background_pressed: Color::from_rgb(0.1, 0.5, 0.9),
    background_disabled: Color::GRAY,
    text_color_active: Color::WHITE,
    border_width: 2.0,
    border_radius: 8.0,
    shadow_offset: Vector::new(2.0, 2.0)
)
```

**All Available Properties:**

- **Active State**: `background_active`, `text_color_active`, `border_color_active`
- **Hovered State**: `background_hovered`, `text_color_hovered`, `border_color_hovered`
- **Pressed State**: `background_pressed`, `text_color_pressed`, `border_color_pressed`
- **Disabled State**: `background_disabled`, `text_color_disabled`, `border_color_disabled`
- **Common**: `border_width`, `border_radius`, `shadow_offset`

**Builder Pattern Alternative:**

```rust
use crate::app::widgets::ButtonBuilder;

let button = ButtonBuilder::new(text("Click"))
    .background_active(Color::BLUE)
    .background_hovered(Color::from_rgb(0.3, 0.7, 1.0))
    .text_color_active(Color::WHITE)
    .border_radius(8.0)
    .build()
    .on_press(Message::Click);
```

**Conditional Button:**

```rust
iced::widget::button("Save")
    .on_press_maybe(
        (!input.is_empty()).then_some(Message::Save)
    )
// Button is disabled if input is empty
```

---

## Container Widgets

### `frame!` - Styled Container

Create containers with borders, shadows, and backgrounds.

**Basic Usage:**

```rust
use crate::frame;

// Simple frame with border
frame!(
    text("Content"),
    border_color = palette.primary,
    border_width = 2.0
)

// With rounded corners
frame!(
    column![/* ... */],
    border_color = palette.primary,
    border_width = 2.0,
    border_radius = 8.0
)
```

**Full Customization:**

```rust
frame!(
    content,                                       // Widget to wrap
    background = color.into(),                     // Background color
    border_color = color,                          // Border color
    shadow_color = color,                          // Shadow color
    text_color = color,                            // Text color
    border_width = 2.0,                            // Border thickness
    border_radius = 8.0,                           // Corner radius
    shadow_blur_radius = 4.0,                      // Shadow blur
    shadow_offset = iced::Vector::new(1.0, 1.0),   // Shadow offset
)
```

**Common Patterns:**

```rust
// Primary frame (for main content areas)
frame!(
    content,
    border_color = palette.primary,
    border_width = 4.0
)

// Secondary frame (for nested content)
let bg_faded = Color::from_rgba(
    bg_color.r,
    bg_color.g,
    bg_color.b,
    0.75
);

frame!(
    content,
    background = bg_faded,
    border_color = palette.primary,
    border_width = 2.0
)

// Card-like frame with shadow
frame!(
    content,
    border_color = palette.primary,
    border_width = 1.0,
    border_radius = 12.0,
    shadow_blur_radius = 8.0,
    shadow_offset = iced::Vector::new(0.0, 4.0)
)
```

**Layout Helpers:**

Frames return `Container`, which has layout methods:

```rust
frame!(content, /* ... */)
    .center_x(Length::Fill)    // Center horizontally
    .align_top(Length::Fill)   // Align to top
    .padding(20)               // Add padding
    .width(Length::Fill)       // Fill width
    .height(Length::Shrink)    // Shrink to content
```

---

## Styling Guide

### Getting Theme Colors

```rust
pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    let theme = app.theme();
    let palette = theme.palette();
    let palette_ext = theme.extended_palette();
    
    // Basic palette
    let primary = palette.primary;
    let success = palette.success;
    let danger = palette.danger;
    let text = palette.text;
    let background = palette.background;
    
    // Extended palette (more variants)
    let primary_base = palette_ext.primary.base.color;
    let primary_strong = palette_ext.primary.strong.color;
    let primary_weak = palette_ext.primary.weak.color;
    
    let success_color = palette_ext.success.base.color;
    let danger_color = palette_ext.danger.base.color;
    
    let bg_base = palette_ext.background.base.color;
    let bg_weak = palette_ext.background.weak.color;
    let bg_strong = palette_ext.background.strong.color;
    
    // ...
}
```

### Creating Faded Colors

```rust
// 75% opacity
let bg_faded = iced::Color::from_rgba(
    bg_color.r,
    bg_color.g,
    bg_color.b,
    0.75
);

// 50% opacity
let semi_transparent = iced::Color { a: color.a * 0.5, ..color };
```

### Common Color Patterns

```rust
// Success actions (save, apply, confirm)
background = palette_ext.success.base.color.into()

// Danger actions (delete, cancel, warning)
background = palette_ext.danger.base.color.into()

// Warning/Edit actions (yellow/orange)
background = iced::Color::from_rgb(0.7, 0.6, 0.0).into()

// Info/Primary actions (default blue)
background = palette_ext.primary.base.color.into()

// Neutral/Disabled
background = palette_ext.background.weak.color.into()
```

---

## Gradients

### `gradient!` - Linear Gradients

Create gradients for backgrounds.

**Basic Usage:**

```rust
use crate::gradient;

// Horizontal gradient (0 degrees)
let grad1 = gradient!(
    Color::from_rgb(1.0, 0.0, 0.0) => 0.0,
    Color::from_rgb(0.0, 0.0, 1.0) => 1.0
);

// Vertical gradient (90 degrees)
let grad2 = gradient!(
    90.0;
    Color::WHITE => 0.0,
    Color::BLACK => 1.0
);

// Multi-stop gradient
let grad3 = gradient!(
    45.0;
    Color::RED => 0.0,
    Color::YELLOW => 0.5,
    Color::BLUE => 1.0
);
```

**With RGB Arrays:**

```rust
let gradient = gradient!(
    90.0;
    [1.0, 0.0, 0.0] => 0.0,
    [0.0, 0.0, 1.0] => 1.0
);
```

**Builder Pattern Alternative:**

```rust
use crate::app::widgets::GradientBuilder;

let gradient = GradientBuilder::new()
    .angle(45.0)
    .stop(Color::RED, 0.0)
    .stop(Color::GREEN, 0.5)
    .stop(Color::BLUE, 1.0)
    .build();
```

**Using in Backgrounds:**

```rust
frame!(
    content,
    background: gradient!(
        90.0;
        Color::from_rgb(0.1, 0.1, 0.2) => 0.0,
        Color::from_rgb(0.2, 0.2, 0.4) => 1.0
    ),
    border_radius: 12.0
)
```

---

## Layout Patterns

### Standard Feature Layout

```rust
pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    let state = &app.state.features.your_feature;
    let theme = app.theme();
    let palette = theme.palette();
    
    // Controls row at top
    let controls = row![
        button!(nerd_text!("Action 1")).on_press(Message::Action1),
        button!(nerd_text!("Action 2")).on_press(Message::Action2),
    ].spacing(10).padding(10);
    
    // Main content
    let content = column![/* ... */]
        .spacing(10)
        .padding(10);
    
    // Wrap in frame
    column![
        frame!(controls, border_color = palette.primary, border_width = 2.0),
        frame!(scrollable(content), border_color = palette.primary, border_width = 2.0),
    ]
    .spacing(10)
    .into()
}
```

### Button Row

```rust
let button_row = row![
    button!(
        nerd_text!(("{} Save", Icon::Apply)),
        background_active: palette_ext.success.base.color,
        border_radius: 8.0
    ).on_press(Message::Save).width(Length::Fill),
    
    button!(
        nerd_text!(("{} Cancel", Icon::Cancel)),
        background_active: palette_ext.danger.base.color,
        border_radius: 8.0
    ).on_press(Message::Cancel).width(Length::Fill),
]
.spacing(10)
.padding(10);
```

### List Items

```rust
let items_col = state.items.iter().fold(
    column![].spacing(10).padding(10),
    |col, item| {
        let item_row = row![
            nerd_text!(("{} {}", Icon::Variable, item.name)),
            horizontal_space(),
            button!(
                icon!(Icon::Edit),
                border_radius: 8.0
            ).on_press(Message::Edit(item.id.clone())),
            button!(
                icon!(Icon::Delete),
                background_active: palette_ext.danger.base.color,
                border_radius: 8.0
            ).on_press(Message::Delete(item.id.clone())),
        ].spacing(10).padding(10);
        
        col.push(frame!(
            item_row,
            background = bg_faded,
            border_color = palette.primary,
            border_width = 1.0
        ))
    }
);
```

---

## Tips & Best Practices

### Do's ✅

- Use the widget macros for consistency
- Get theme colors from `app.theme()`
- Use `border_radius = 8.0` for modern rounded corners
- Use semantic colors (success for save, danger for delete)
- Add spacing and padding for breathing room
- Use `Length::Fill` to make buttons equal width

### Don'ts ❌

- Don't hardcode colors - use the theme
- Don't forget to specify message handlers (`.on_press()`)
- Don't nest too many frames - it gets cluttered
- Don't use tiny text (< 12px) - readability matters

### Accessibility

- Use sufficient color contrast
- Provide text alternatives for icons
- Make clickable areas large enough (min 32x32px)
- Use semantic colors (red for danger, green for success)

---

## Examples

### Example 1: Form Layout

```rust
let form = column![
    text("User Settings").size(24),
    
    text("Name:"),
    text_input("Enter name", &state.name)
        .on_input(Message::NameChanged),
    
    text("Email:"),
    text_input("Enter email", &state.email)
        .on_input(Message::EmailChanged),
    
    row![
        button!(
            nerd_text!("Save"),
            background_active: palette_ext.success.base.color,
            border_radius: 8.0
        ).on_press(Message::Save).width(Length::Fill),
        
        button!(
            nerd_text!("Cancel"),
            border_radius: 8.0
        ).on_press(Message::Cancel).width(Length::Fill),
    ].spacing(10),
]
.spacing(10)
.padding(20);

frame!(
    form,
    border_color = palette.primary,
    border_width = 2.0,
    border_radius = 8.0
).into()
```

### Example 2: Icon Grid

```rust
let icon_size = 32.0;

let icons_row = row![
    icon!(Icon::Settings, size = icon_size),
    icon!(Icon::Edit, size = icon_size),
    icon!(Icon::Delete, size = icon_size, color = palette_ext.danger.base.color),
    icon!(Icon::Copy, size = icon_size),
    icon!(Icon::Variable, size = icon_size),
]
.spacing(20);

frame!(icons_row, border_color = palette.primary, border_width = 1.0)
    .padding(20)
```

### Example 3: Status Card

```rust
let status = if state.is_connected {
    ("Connected", Icon::Apply, palette_ext.success.base.color)
} else {
    ("Disconnected", Icon::Cancel, palette_ext.danger.base.color)
};

let card = column![
    row![
        icon!(status.1, size = 24.0, color = status.2),
        nerd_text!(status.0, size = 18.0, color = status.2),
    ].spacing(10),
    
    text("Last sync: 5 minutes ago").size(12.0),
]
.spacing(5)
.padding(15);

frame!(
    card,
    background = Color::from_rgba(status.2.r, status.2.g, status.2.b, 0.1),
    border_color = status.2,
    border_width = 2.0,
    border_radius = 8.0
)
```

---

## Next Steps

- See [ADDING_FEATURES.md](ADDING_FEATURES.md) for complete examples
- Check `src/app/features/root.rs` for real-world usage
- Experiment with different styles in your features
