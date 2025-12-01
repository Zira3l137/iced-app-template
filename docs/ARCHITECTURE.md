# Architecture Overview

This document explains the structure and design patterns used in `{{project-name}}`.

## Table of Contents

1. [High-Level Architecture](#high-level-architecture)
2. [Directory Structure](#directory-structure)
3. [Core Concepts](#core-concepts)
4. [Data Flow](#data-flow)
5. [Module Breakdown](#module-breakdown)

---

## High-Level Architecture

`{{project-name}}` follows a **unidirectional data flow architecture** inspired by The Elm Architecture (TEA), which Iced implements:

```
┌─────────────┐
│    View     │  Renders UI based on current state
└──────┬──────┘
       │
       │ User Interaction
       ▼
┌─────────────┐
│   Message   │  Describes what happened
└──────┬──────┘
       │
       │ Dispatched
       ▼
┌─────────────┐
│   Update    │  Processes message, updates state, returns tasks
└──────┬──────┘
       │
       │ New State
       ▼
┌─────────────┐
│    State    │  Application data
└─────────────┘
```

**Key Principles:**
- **Immutable message passing** - All changes flow through typed messages
- **Pure view functions** - Views don't have side effects, just render state
- **Controlled side effects** - I/O operations return `Task`s, not executed directly
- **Type safety** - Rust's type system prevents many runtime errors

---

## Directory Structure

```
{{project-name}}/
├── src/
│   ├── main.rs              # Entry point, CLI parsing, logger setup
│   ├── app/                 # Application core
│   │   ├── mod.rs           # Main application struct and run loop
│   │   ├── macros.rs        # Feature registration macro system
│   │   ├── message.rs       # Message type definitions
│   │   ├── state.rs         # Application state structure
│   │   ├── session.rs       # Persistent session data
│   │   ├── theme.rs         # Theme definitions
│   │   ├── windows.rs       # Window management
│   │   ├── features/        # Feature modules (windows/views)
│   │   │   ├── mod.rs       # Feature registration
│   │   │   ├── root.rs      # Main application view / primary feature
│   │   │   └── settings.rs  # Settings/about view
│   │   └── widgets/         # Reusable UI components
│   │       ├── mod.rs       # Widget utilities and macros
│   │       └── clickable_text.rs
│   ├── cli/                 # Command-line argument parsing
│   ├── core/                # Core types and constants
│   ├── error.rs             # Error types
│   ├── logger.rs            # Logging configuration
│   ├── persistence/         # Data persistence (sessions, settings)
│   ├── platform/            # Platform-specific code
│   └── utils.rs             # General utilities
├── resources/               # Fonts, icons, assets
├── docs/                    # Documentation
└── Cargo.toml               # Dependencies and metadata
```

---

## Core Concepts

### 1. Application Structure

The `Application` struct in `src/app/mod.rs` is the heart of the app:

```rust
pub struct Application {
    pub session: ApplicationSession,  // Persistent data (theme, window positions)
    pub state: ApplicationState,      // Runtime state (UI state, feature states)
}
```

### 2. State Management

State is divided into two main categories:

**Session (Persistent)**
```rust
pub struct ApplicationSession {
    pub theme_selected: Option<String>,
}
```
- Saved to disk between runs
- Located in `src/app/session.rs`
- Serialized as JSON

**State (Runtime)**
```rust
pub struct ApplicationState {
    pub ui: UiState,              // UI-level state (themes, windows)
    pub features: FeaturesState,  // Feature-specific state
}
```
- Created fresh on each launch
- Located in `src/app/state.rs`
- Macro-generated from feature registrations

### 3. Message System

Messages are **events** that describe what happened. They flow in a hierarchy:

```rust
pub enum AppMessage {
    Window(WindowMessage),      // Window management
    System(SystemMessage),      // System-level operations
    Feature(FeatureMessage),    // Feature-specific messages
}
```

**Example Flow:**
```
User clicks "Copy" button
    → Message::CopyToClipboard(value)
    → FeatureMessage::Root(Message::CopyToClipboard(value))
    → AppMessage::Feature(FeatureMessage::Root(...))
    → update() processes it
    → Returns Task to write to clipboard
```

### 4. Features (Windows/Views)

A **feature** is a self-contained module that implements a window or view:

**Required Components:**
```rust
// State - Feature's data
#[derive(Debug, Default)]
pub struct State { /* ... */ }

// Message - Events this feature can handle
#[derive(Debug, Clone)]
pub enum Message { /* ... */ }

// update - Process messages, return tasks
pub fn update(state: &mut State, msg: Message) -> AppTask { /* ... */ }

// view - Render UI based on state
pub fn view(app: &Application) -> Element<Message> { /* ... */ }
```

**Registration:**
Features are registered using the `register_features!` macro which automatically generates routing code.

---

## Data Flow

### Startup Sequence

```
1. main()
   ├─> Parse CLI arguments
   ├─> Setup logger
   └─> app::run()

2. app::run()
   └─> iced::daemon(...)
       └─> Application::new()
           ├─> Load session from disk
           ├─> Initialize state
           ├─> Load custom font
           └─> Open main window

3. Application loop starts
   └─> View → Message → Update → View...
```

### Message Processing

```
1. User interacts with UI
   └─> View generates Message

2. Message bubbles up to Application::update()
   ├─> AppMessage::Window → Handle window operations
   ├─> AppMessage::System → Handle system commands
   └─> AppMessage::Feature → Route to feature's update()

3. Update function returns Task
   ├─> Task::none() - No side effects
   ├─> Task::done(message) - Dispatch another message
   └─> Other tasks - I/O, timers, etc.

4. State updated, view re-rendered
```

### Window Management

```
Opening a window:
1. WindowMessage::Open(ApplicationWindow::Settings)
2. windows::invoke_window()
   ├─> Create window with iced::window::open()
   ├─> Store window info in state
   └─> Return task

Closing a window:
1. User clicks X or WindowMessage::Close(id)
2. windows::close_window()
   ├─> Mark window as closed
   ├─> iced::window::close(id)
   └─> Check if should exit app
```

---

## Module Breakdown

### `src/app/mod.rs`

**Main application logic:**
- `Application` struct - Holds session and state
- `new()` - Initialization
- `update()` - Message routing
- `view()` - Dispatches to window views
- `theme()` - Current theme lookup
- `subscription()` - Event listeners
- `run()` - Launch the Iced runtime

### `src/app/macros.rs`

**Feature registration system:**
- `register_features!` - Macro to register features
- Generates `FeatureMessage` enum
- Generates `FeaturesState` struct
- Generates window configuration methods
- Generates routing functions

See [ADDING_FEATURES.md](ADDING_FEATURES.md) for usage.

### `src/app/message.rs`

**Message type definitions:**
- `AppMessage` - Top-level message enum
- `WindowMessage` - Window operations (open, close)
- `SystemMessage` - System operations (execute command, exit)
- Feature messages defined in `features/mod.rs`

### `src/app/state.rs`

**Application state:**
- `ApplicationState` - Root state struct
- `UiState` - UI-level state (themes, windows)
- `FeaturesState` - Generated by macro, holds feature states

### `src/app/windows.rs`

**Window management:**
- `ApplicationWindow` enum - Window types
- `WindowInfo` - Window metadata
- `invoke_window()` - Open a window
- `close_window()` - Close a window
- `exit_application()` - Check if should exit

### `src/app/features/`

**Feature modules:**
Each feature is a self-contained module with:
- State struct
- Message enum
- update() function
- view() function

**Example features in the template:**
- `root` - Example primary application view
- `settings` - Example settings and about page

### `src/app/widgets/`

**Reusable UI components:**
- `mod.rs` - Builder pattern implementations for widgets
- `macros.rs` - Convenience macros wrapping builders
- `clickable_text.rs` - Hoverable clickable text widget

**Available macros:**
- `icon!()` - Render Nerd Font icons
- `nerd_text!()` - Text with custom font
- `clickable_text!()` - Hoverable clickable text
- `frame!()` - Styled container
- `button!()` - Customizable button with state styling
- `gradient!()` - Linear gradients for backgrounds
- `text_input!()` - Linear gradients for backgrounds

**Builder Pattern:**
All widgets have corresponding builder structs (`IconBuilder`, `ButtonBuilder`, etc.) 
for more advanced customization. Macros provide a simpler API for common cases.

### `src/core/`

**Core types and constants:**
- `types.rs` - Type aliases (Lookup, Icon enum)
- `constants.rs` - App metadata and paths

### `src/persistence/`

**Data persistence:**
- Session saving/loading (JSON to local app data)

### `src/platform/`

**Platform-specific code:**
- Command execution (open URLs, etc.)

---

## Key Design Decisions

### Why Macro-Based Feature Registration?

**Problem:** Adding a new feature required editing 5 files with repetitive boilerplate.

**Solution:** The `register_features!` macro generates all boilerplate automatically.

**Benefits:**
- Add features by editing 2 locations (module + registration)
- Impossible to forget a step
- Centralized configuration
- Compile-time generation (zero runtime overhead)

### Why Separate Session from State?

**Session** persists between runs (user preferences).
**State** is ephemeral (runtime UI state).

This separation makes it clear what gets saved and what doesn't.

### Why Feature-Based Architecture?

Each feature is isolated, making it easy to:
- Add new features without touching existing ones
- Test features independently
- Understand the codebase (each feature is self-contained)

---

## Next Steps

- [Adding Features Guide](ADDING_FEATURES.md) - Learn how to add new windows/features
- [Contributing Guide](CONTRIBUTING.md) - How to contribute to the project
- [Widget Guide](WIDGETS.md) - Using the widget macro system
