# Adding New Features

This guide shows you how to add new windows/features to your `{{project-name}}` application.

## Table of Contents

1. [What is a Feature?](#what-is-a-feature)
2. [Quick Start](#quick-start)
3. [Step-by-Step Guide](#step-by-step-guide)
4. [Feature Template](#feature-template)
5. [Common Patterns](#common-patterns)
6. [Examples](#examples)

---

## What is a Feature?

A **feature** in `{{project-name}}` is a self-contained module that typically represents:
- A window (like the main application view or a settings dialog)
- A major piece of functionality with its own UI

Each feature has:
- **State** - The data it needs
- **Message** - Events it can handle
- **update()** - Logic to process messages
- **view()** - UI rendering

---

## Quick Start

**Adding a new feature takes 3 steps:**

### 1. Create the Feature Module

Create `src/app/features/my_feature.rs`:

```rust
use super::super::AppTask;
use crate::{frame, nerd_text, button};
use iced::widget::{column, text};

#[derive(Debug, Default)]
pub struct State {
    pub counter: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
}

pub fn update(state: &mut State, msg: Message) -> AppTask {
    match msg {
        Message::Increment => {
            state.counter += 1;
            AppTask::none()
        }
        Message::Decrement => {
            state.counter -= 1;
            AppTask::none()
        }
    }
}

pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    let state = &app.state.features.my_feature;
    
    column![
        text(format!("Counter: {}", state.counter)),
        button!(nerd_text!("Increment")).on_press(Message::Increment),
        button!(nerd_text!("Decrement")).on_press(Message::Decrement),
    ]
    .into()
}
```

### 2. Register the Feature

Edit `src/app/features/mod.rs`:

```rust
pub mod root;
pub mod settings;
pub mod my_feature;  // 1. Add module declaration

use crate::register_features;

register_features! {
    Root => root { 768.0, 768.0, Centered },
    Settings => settings { 768.0, 460.0, Centered },
    MyFeature => my_feature { 600.0, 400.0, Centered },  // 2. Register it
}
```

### 3. Add Window Type

Edit `src/app/windows.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Hash, Default, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum ApplicationWindow {
    #[default]
    Root,
    Settings,
    MyFeature,  // Add your window variant
}
```

**That's it!** The macro generates all the boilerplate automatically.

---

## Step-by-Step Guide

### Step 1: Plan Your Feature

Before writing code, answer these questions:

**What data does it need?**
```rust
// Example: A search feature needs:
pub struct State {
    pub search_query: String,
    pub search_results: Vec<String>,
    pub is_searching: bool,
}
```

**What actions can users take?**
```rust
// Example: Search feature actions
pub enum Message {
    SearchInputChanged(String),
    PerformSearch,
    ClearSearch,
    SelectResult(usize),
}
```

**What window size does it need?**
```rust
// Small dialog: 400x300
// Medium window: 600x500
// Large window: 800x600
// Full-featured: 768x768+
```

### Step 2: Create the Feature Module

Create `src/app/features/your_feature.rs` with this structure:

```rust
// Imports
use super::super::AppTask;
use iced::widget::{column, row, text};

// State - Data your feature needs
#[derive(Debug, Default)]
pub struct State {
    // Your fields here
}

// Message - Events your feature handles
#[derive(Debug, Clone)]
pub enum Message {
    // Your messages here
}

// Update - Process messages, update state
pub fn update(state: &mut State, msg: Message) -> AppTask {
    match msg {
        // Handle your messages
    }
}

// View - Render UI
pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    // Build your UI
    text("Hello from your feature!").into()
}
```

**Important traits:**
- `State` must implement `Debug` and `Default`
- `Message` must implement `Debug` and `Clone`

### Step 3: Implement State

Define what data your feature needs:

```rust
#[derive(Debug, Default)]
pub struct State {
    // Example: Input fields
    pub input_text: String,
    pub selected_index: usize,
    
    // Example: Collections
    pub items: Vec<String>,
    
    // Example: Flags
    pub is_loading: bool,
    pub show_advanced: bool,
}
```

**Tips:**
- Use `String` for text inputs
- Use `Vec<T>` for lists
- Use `Option<T>` for nullable values
- Use `bool` for toggles/flags
- Use custom types from `src/core/types.rs` (e.g., `Lookup<K, V>`)

### Step 4: Implement Messages

Define all actions users can take:

```rust
#[derive(Debug, Clone)]
pub enum Message {
    // Input events
    InputChanged(String),
    SelectionChanged(usize),
    
    // Button clicks
    SaveClicked,
    CancelClicked,
    DeleteClicked(String),
    
    // Async operations
    LoadData,
    DataLoaded(Vec<String>),
}
```

**Message naming conventions:**
- Use descriptive names: `SaveClicked` not `Save`
- Include data in enum variants: `DeleteClicked(String)`

### Step 5: Implement Update

Process messages and return tasks:

```rust
pub fn update(state: &mut State, msg: Message) -> AppTask {
    match msg {
        Message::InputChanged(text) => {
            state.input_text = text;
            AppTask::none()  // No side effects
        }
        
        Message::SaveClicked => {
            // Do something with state.input_text
            AppTask::done(Message::CancelClicked.into())  // Chain another message
        }
        
        Message::CopyToClipboard(text) => {
            iced::clipboard::write(text)  // Built-in clipboard task
        }
    }
}
```

**Common task patterns:**

```rust
// No side effects
AppTask::none()

// Dispatch another message immediately
AppTask::done(Message::SomeMessage.into())

// Copy to clipboard
iced::clipboard::write(text)

// Open a window
use crate::app::message::{AppMessage, WindowMessage};
use crate::app::windows::ApplicationWindow;

AppTask::done(
    AppMessage::Window(WindowMessage::Open(ApplicationWindow::Settings))
)

// Execute system command
use crate::app::message::SystemMessage;

AppTask::done(
    SystemMessage::ExecuteCommand("explorer".to_owned(), vec![url]).into()
)
```

### Step 6: Implement View

Build your UI using Iced widgets:

```rust
pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    // 1. Get your feature's state
    let state = &app.state.features.your_feature;
    
    // 2. Get theme for styling
    let theme = app.theme();
    let palette = theme.palette();
    
    // 3. Build UI
    use iced::widget::{button, column, row, text, text_input};
    
    column![
        text("My Feature").size(24),
        
        text_input("Placeholder", &state.input_text)
            .on_input(Message::InputChanged),
        
        button("Save")
            .on_press(Message::SaveClicked),
    ]
    .spacing(10)
    .padding(20)
    .into()
}
```

**Using custom widgets:**

```rust
use crate::{frame, icon, nerd_text, button, gradient};
use crate::core::types::Icon;

// Styled button with state-specific colors
button!(
    nerd_text!(("{} Save", Icon::Apply)),
    background_active: palette.success,
    background_hovered: palette.success * 1.2,
    border_radius: 8.0
).on_press(Message::SaveClicked)

// Frame (container with border)
frame!(
    text("Content"),
    border_color: palette.primary,
    border_width: 2.0,
    border_radius: 8.0
)

// Icon
icon!(Icon::Settings, size: 24, color: palette.primary)

// Gradient background for hero sections
let hero_bg = gradient!(
    palette.primary => 0.0,
    palette.background => 1.0
);
```

### Step 7: Register the Feature

Edit `src/app/features/mod.rs`:

```rust
pub mod root;
pub mod settings;
pub mod your_feature;  // Add this

use crate::register_features;

register_features! {
    Root => root { 768.0, 768.0, Centered },
    Settings => settings { 768.0, 460.0, Centered },
    YourFeature => your_feature { 600.0, 400.0, Centered },  // Add this
    // WindowVariant => module_name { width, height, position }
}
```

**Position options:**
- `Centered` - Center of screen
- `Default` - OS default
- `Specific(x, y)` - Exact coordinates

### Step 8: Add Window Variant

Edit `src/app/windows.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Hash, Default, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum ApplicationWindow {
    #[default]
    Root,
    Settings,
    YourFeature,  // Add your variant
}
```

### Step 9: Add a Way to Open It

In another feature (e.g., `root.rs`), add a button:

```rust
use crate::app::windows::ApplicationWindow;
use crate::app::message::{AppMessage, WindowMessage};

// In your view:
button!(nerd_text!("Open My Feature"))
    .on_press_maybe(Some(
        AppMessage::Window(WindowMessage::Open(ApplicationWindow::YourFeature))
    ))
```

**Note:** This requires converting your message to `AppMessage`. See [Common Patterns](#common-patterns) below.

---

## Feature Template

Copy this template to start a new feature quickly:

```rust
// src/app/features/template.rs

use super::super::AppTask;
use crate::{frame, nerd_text, button};
use crate::core::types::Icon;

use iced::widget::{column, row, text, text_input};
use iced::Length;

// ============================================================================
// State
// ============================================================================

#[derive(Debug, Default)]
pub struct State {
    // TODO: Add your fields
}

// ============================================================================
// Message
// ============================================================================

#[derive(Debug, Clone)]
pub enum Message {
    // TODO: Add your messages
    DoNothing,
}

// ============================================================================
// Update
// ============================================================================

pub fn update(state: &mut State, msg: Message) -> AppTask {
    match msg {
        Message::DoNothing => AppTask::none(),
        // TODO: Handle your messages
    }
}

// ============================================================================
// View
// ============================================================================

pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    let state = &app.state.features.template;  // TODO: Change 'template'
    let theme = app.theme();
    let palette = theme.palette();
    
    // TODO: Build your UI
    let content = column![
        text("Template Feature").size(24),
        text("Replace this with your UI"),
    ]
    .spacing(10)
    .padding(20);
    
    frame!(
        content,
        border_color = palette.primary,
        border_width = 2.0
    )
    .into()
}
```

---

## Common Patterns

### Pattern 1: Opening Another Window

```rust
// In your Message enum
pub enum Message {
    OpenSettings,
}

// In update
use crate::app::message::{AppMessage, WindowMessage};
use crate::app::windows::ApplicationWindow;

Message::OpenSettings => {
    AppTask::done(
        AppMessage::Window(WindowMessage::Open(ApplicationWindow::Settings))
    )
}
```

### Pattern 2: Accessing Theme/Palette

```rust
pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    let theme = app.theme();
    let palette = theme.palette();
    let palette_ext = theme.extended_palette();
    
    // Use palette colors
    let success_color = palette_ext.success.base.color;
    let danger_color = palette_ext.danger.base.color;
    let primary_color = palette.primary;
    
    // ...
}
```

### Pattern 3: Iterating with Fold

Build a column/row from a collection:

```rust
let items_column = state.items.iter().fold(
    column![].spacing(5),
    |col, item| {
        col.push(
            button(text(item))
                .on_press(Message::ItemClicked(item.clone()))
        )
    }
);
```

### Pattern 4: Conditional UI

```rust
let content = if state.is_loading {
    column![text("Loading...")]
} else {
    column![
        text("Loaded!"),
        // ... more UI
    ]
};
```

### Pattern 5: Optional Button Press

```rust
button("Save")
    .on_press_maybe(
        state.input_text.is_empty()
            .then(|| None)
            .unwrap_or(Some(Message::Save))
    )
// Button is disabled if input is empty
```

### Pattern 6: System Commands

```rust
use crate::app::message::SystemMessage;

Message::OpenUrl(url) => {
    AppTask::done(
        SystemMessage::ExecuteCommand(
            "explorer".to_owned(),  // or "xdg-open" on Linux
            vec![url]
        ).into()
    )
}
```

---

## Tips & Best Practices

### Do's ✅

- Keep features focused and self-contained
- Use `Default` for initial state
- Use descriptive message names
- Handle all messages in `update()`
- Use the widget macros for consistent styling
- Access state via `app.state.features.your_feature`
- Use `AppTask::none()` when no side effects needed

### Don'ts ❌

- Don't perform I/O in `view()` - views should be pure
- Don't store derived data in state - compute it in `view()`
- Don't forget to add `Debug` and `Clone` derives
- Don't modify state in `view()` - only in `update()`
- Don't use `unwrap()` on user input - handle errors gracefully

### Performance Tips

- Use `scrollable()` for long lists
- Use `fold()` instead of collecting into `Vec<Element>`
- Lazy-load data only when needed
- Keep state minimal - derive what you can in `view()`

---

## Next Steps

- Study existing features: `src/app/features/root.rs` and `settings.rs`
- Read [ARCHITECTURE.md](ARCHITECTURE.md) for deeper understanding
- Read [WIDGETS.md](WIDGETS.md) for widget macro documentation
- Check [CONTRIBUTING.md](CONTRIBUTING.md) before submitting PRs
