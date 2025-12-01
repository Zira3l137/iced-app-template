# Contributing to {{project-name}}

Thank you for your interest in contributing! This guide will help you get started.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Code Style](#code-style)
5. [Submitting Changes](#submitting-changes)
6. [Project Structure](#project-structure)

---

## Code of Conduct

Be respectful, constructive, and professional in all interactions.

---

## Getting Started

### Prerequisites

- **Rust** 2024 edition or later
- **Git** for version control
- A code editor (VS Code with rust-analyzer recommended)

### Fork and Clone (for the template itself)

If you want to contribute improvements to the **template**:

```bash
# Fork the template repository on GitHub, then:
git clone {{repository-url}}.git
cd {{project-name}}

# Add upstream remote (original template)
git remote add upstream {{repository-url}}.git
```

### Working on a project generated from this template

If you're contributing to an **application** that was created using this template, follow that application's own repository URL and contribution guidelines, but the development workflow and code style described in this document still apply.

### Build and Run

```bash
# Build
cargo build

# Run
cargo run

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

---

## Development Workflow

### 1. Create a Branch

```bash
# Update your main branch
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/your-feature-name
```

**Branch naming conventions:**
- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring

### 2. Make Changes

- Follow the [Code Style](#code-style) guidelines
- Write clear commit messages
- Test your changes locally
- Update documentation if needed

### 3. Commit Changes

```bash
git add .
git commit -m "Add: Brief description of change"
```

**Commit message prefixes:**
- `Add:` - New feature or file
- `Fix:` - Bug fix
- `Update:` - Modify existing feature
- `Remove:` - Delete code or files
- `Docs:` - Documentation only
- `Refactor:` - Code restructuring

### 4. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

---

## Code Style

### Rust Style

Follow standard Rust conventions:

```rust
// Use rustfmt (automatic)
cargo fmt

// Check for issues
cargo clippy
```

**Key points:**
- Use snake_case for functions and variables
- Use PascalCase for types and enum variants
- Use SCREAMING_SNAKE_CASE for constants
- Use 4 spaces for indentation (not tabs)
- Keep lines under 100 characters when practical
- Add doc comments (`///`) for public items

### Project Conventions

**State structs:**
```rust
#[derive(Debug, Default)]
pub struct State {
    pub field_name: Type,
}
```
- Always derive `Debug` and `Default`
- Use `pub` for fields accessed from `view()`

**Message enums:**
```rust
#[derive(Debug, Clone)]
pub enum Message {
    ActionPerformed,
    ValueChanged(String),
}
```
- Always derive `Debug` and `Clone`

**Update functions:**
```rust
pub fn update(state: &mut State, msg: Message) -> AppTask {
    match msg {
        Message::ActionPerformed => {
            // Logic here
            AppTask::none()
        }
    }
}
```
- Return `AppTask`, not `()` or `Result`
- Use `AppTask::none()` when no side effects

**View functions:**
```rust
pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    let state = &app.state.features.your_feature;
    // Build UI
    text("Hello").into()
}
```
- Always take `app: &Application` parameter
- Always return `iced::Element<'a, Message>`
- Call `.into()` at the end

### File Organization

Within a feature file:

```rust
// 1. Imports
use super::super::AppTask;
use iced::widget::{column, text};

// 2. State
#[derive(Debug, Default)]
pub struct State { /* ... */ }

// 3. Message
#[derive(Debug, Clone)]
pub enum Message { /* ... */ }

// 4. Update
pub fn update(state: &mut State, msg: Message) -> AppTask { /* ... */ }

// 5. View
pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> { /* ... */ }

// 6. Helper functions (if any)
fn helper_function() { /* ... */ }
```

---

## Submitting Changes

### Before Submitting

**Checklist:**
- [ ] Code compiles without errors (`cargo build`)
- [ ] Code passes all tests (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated (if applicable)
- [ ] Commit messages are clear
- [ ] Branch is up to date with main

### Pull Request Guidelines

**Title:**
- Clear and descriptive
- Use the same prefix as commit messages
- Example: "Add: Environment variable export feature"

**Description should include:**
- What the PR does
- Why this change is needed
- How to test it
- Screenshots (for UI changes)
- Related issues (if any)

**PR Template:**
```markdown
## Description
Brief description of changes.

## Type of Change
- [ ] New feature
- [ ] Bug fix
- [ ] Documentation update
- [ ] Refactoring

## How to Test
1. Step one
2. Step two
3. Expected result

## Screenshots
(If applicable)

## Related Issues
Fixes #123
```

### Review Process

1. Automated checks run (build, clippy, tests)
2. Code review by maintainers
3. Address feedback if needed
4. Once approved, PR will be merged

---

## Project Structure

### Key Directories

```
src/
├── app/              # Application core (main contribution area)
│   ├── features/     # Add new features here
│   ├── widgets/      # Add reusable widgets here
│   ├── macros.rs     # Don't modify unless adding macro features
│   └── ...
├── core/             # Core types and constants
├── cli/              # CLI argument parsing
├── persistence/      # Data persistence
└── platform/         # Platform-specific code
```

### Where to Contribute

**Adding a new feature/window:**
→ See [ADDING_FEATURES.md](ADDING_FEATURES.md)
→ Create file in `src/app/features/`
→ Register in `src/app/features/mod.rs`

**Adding a new widget:**
→ See [WIDGETS.md](WIDGETS.md)
→ Add to `src/app/widgets/mod.rs`

**Fixing a bug:**
→ Find the relevant file based on error
→ Fix and test
→ Submit PR with clear explanation

**Improving documentation:**
→ Update relevant `.md` file in `docs/`
→ Keep examples up-to-date

---

## Common Tasks

### Adding a Dependency

Edit `Cargo.toml`:

```toml
[dependencies]
new-crate = "1.0"
```

Then run `cargo build` to fetch it.

### Adding a New Icon

1. Find Unicode codepoint in [Nerd Fonts cheat sheet](https://www.nerdfonts.com/cheat-sheet)
2. Add to `src/core/types.rs`:

```rust
pub enum Icon {
    // ...
    NewIcon,  // 
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let codepoint = match self {
            // ...
            Icon::NewIcon => '\u{YOUR_CODEPOINT}',
        };
        write!(f, "{}", codepoint)
    }
}
```

### Running with Logging

```bash
# Debug level
RUST_LOG=debug cargo run

# Trace level (verbose)
RUST_LOG=trace cargo run

# With CLI flag
cargo run -- -v 5
```

### Debugging Tips

**Use `dbg!()` macro:**
```rust
dbg!(&state.some_field);
```

**Add tracing:**
```rust
tracing::debug!("Processing message: {:?}", msg);
tracing::info!("User clicked button");
tracing::error!("Failed to load file: {}", err);
```

**Check generated macro code:**
```bash
cargo install cargo-expand
cargo expand app::features
```

---

## Getting Help

- Read the docs: [docs/](docs/)
- Check existing issues on GitHub
- Look at existing code for examples
- Ask questions in GitHub Discussions

---

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to {{project-name}}! 🎉
