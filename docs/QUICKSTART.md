# Quick Start Guide

Get up and running with `{{project-name}}` development in 5 minutes.

**Built with Iced 0.14.0 and `anyhow`-based error handling.**

## For Users

Because this is a template, the concrete behavior of the final application depends on how you customize it.
Refer to the documentation of the application built from this template for feature-specific usage instructions.

### Installation

You typically won't work directly in this template repository.
Instead, you either:
- Generate a new project using **cargo-generate**, or
- Clone a repository that was already generated from this template.

#### Option 1: Using cargo-generate (recommended)

```bash
cargo install cargo-generate

cargo generate \
  --git {{repository-url}} \
  --name my-iced-app

cd my-iced-app
```

#### Option 2: Cloning an app built from this template

```bash
git clone {{repository-url}}.git
cd {{project-name}}
```

Then build and run:

```bash
cargo build --release
./target/release/{{project-name}}
```

---

## For Developers

### Setup (< 2 minutes)

```bash
# 1. Clone
git clone {{repository-url}}.git
cd {{project-name}}

# 2. Build and run
cargo run

# That's it! The app should launch.
```

### Understanding the Code (5 minutes)

**Project structure:**
```
src/
├── main.rs              # Entry point
├── app/
│   ├── features/        # ← ADD NEW FEATURES HERE
│   │   ├── root.rs      # Example main view / primary feature
│   │   └── settings.rs  # Example settings window
│   ├── widgets/         # Reusable UI components
│   └── macros.rs        # Feature registration system
└── core/                # Types and constants
```

**Key files to read:**
1. `src/app/features/root.rs` - Example of a complete feature
2. `docs/ADDING_FEATURES.md` - How to add your own features
3. `docs/ARCHITECTURE.md` - Understand the design
4. `crates/{{project-name}}-core/src/error.rs` - Shared `anyhow::Result` alias + helpers

### Adding Your First Feature (10 minutes)

**Step 1:** Create `src/app/features/hello.rs`

```rust
use super::super::AppTask;
use iced::widget::{column, text};

#[derive(Debug, Default)]
pub struct State {
    pub count: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
}

pub fn update(state: &mut State, msg: Message) -> AppTask {
    match msg {
        Message::Increment => state.count += 1,
    }
    AppTask::none()
}

pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    let state = &app.state.features.hello;
    
    column![
        text(format!("Count: {}", state.count)),
        iced::widget::button("+").on_press(Message::Increment),
    ]
    .into()
}
```

**Step 2:** Register in `src/app/features/mod.rs`

```rust
pub mod root;
pub mod settings;
pub mod hello;  // Add this line

use crate::register_features;

register_features! {
    Root => root { 768.0, 768.0, Centered },
    Settings => settings { 768.0, 460.0, Centered },
    Hello => hello { 400.0, 300.0, Centered },  // Add this line
}
```

**Step 3:** Add window type in `src/app/windows.rs`

```rust
pub enum ApplicationWindow {
    #[default]
    Root,
    Settings,
    Hello,  // Add this line
}
```

**Step 4:** Run and test

```bash
cargo run
```

Open the window programmatically or add a button in `root.rs`:

```rust
use crate::app::message::{AppMessage, WindowMessage};
use crate::app::windows::ApplicationWindow;

// Add this button in root.rs view:
button!(nerd_text!("Open Hello"))
    .on_press_maybe(Some(
        AppMessage::Window(WindowMessage::Open(ApplicationWindow::Hello))
    ))
```

**Done!** You've added your first feature.

---

## Common Commands

```bash
# Development
cargo build              # Build debug version
cargo run                # Run debug version
cargo run -- -v 5        # Run with verbose logging

# Testing
cargo test               # Run tests
cargo check              # Fast check without building

# Quality
cargo fmt                # Format code
cargo clippy             # Lint code

# Release
cargo build --release    # Optimized build
cargo run --release      # Run optimized version
```

---

## Helpful Tips

### Use Rust Analyzer

Install the `rust-analyzer` extension in VS Code for:
- Auto-completion
- Inline errors
- Code navigation
- Refactoring tools

### Debug Logging

```bash
# See what's happening
RUST_LOG=debug cargo run

# Even more verbose
RUST_LOG=trace cargo run
```

### Hot Tips

- Check `docs/` for detailed guides
- Look at existing features for examples
- Use `cargo expand app::features` to see generated code

---

## Next Steps

**Essential reading:**
1. ✅ [ARCHITECTURE.md](ARCHITECTURE.md) - Understand the design (15 min)
2. ✅ [ADDING_FEATURES.md](ADDING_FEATURES.md) - Learn to add features (20 min)
3. ✅ [WIDGETS.md](WIDGETS.md) - Master the widget system (10 min)

**Optional reading:**
- [CONTRIBUTING.md](CONTRIBUTING.md) - Before submitting PRs
- Existing code in `src/app/features/` - Real examples

---

## Resources

- [**Unofficial Iced Tutorial:**](https://github.com/fogarecious/iced_tutorial)
- [**Rust Book:**](https://doc.rust-lang.org/book)
- [**Nerd Fonts:**](https://www.nerdfonts.com/cheat-sheet)

---

**Ready to code?** Start with [ADDING_FEATURES.md](ADDING_FEATURES.md)!
