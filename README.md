# iced-app-template

A scalable Rust desktop application template built on [Iced](https://github.com/iced-rs/iced) using the daemon model. Designed for multi-window apps where adding a new feature requires touching as little of the codebase as possible.

## Features

- **Feature-module architecture** — each feature is a self-contained module with its own `State`, `Message`, `Context`, and `view`/`update` functions
- **Code-generation macro** — `register_features!` generates the dispatch boilerplate; adding a feature is a single line
- **Borrow-safe cross-feature dispatch** — `ContextMut::new(&mut App)` takes exclusive access only within a single `match` arm, so no aliasing occurs
- **Cross-feature effects via `Task`** — features never call each other directly; side-effects are returned as tasks
- **Persistent state** — a `Persistent` trait separates runtime state from disk-serialized state
- **Runtime asset loading** — config (TOML), locales, fonts, and icon are all loaded before the daemon starts

## Project layout

```
crates/          # workspace crates
docs/            # architecture decision records
resources/       # fonts, icons
themes/          # theme definitions
app_config.toml  # application configuration
state.toml       # persistent state file
```

## Getting started

This template is intended to be scaffolded with [`cargo-generate`](https://github.com/cargo-generate/cargo-generate):

```sh
cargo generate --git https://github.com/Zira3l137/iced-app-template
```

Then build normally:

```sh
cargo build
```

## Adding a window

Windows are registered with the `register_windows!` macro, which generates a `Window` enum and implements `title`, `settings`, `view`, and `input` dispatch for each variant:

```rust
register_windows!(Main {
    settings: Settings {
        size: Size::new(800.0, 600.0),
        exit_on_close_request: false,
        transparent: true,
        ..Default::default()
    },
    view_handler: main::view,
    input_handler: main::input,
    context: main::Context::new
});
```

Each window entry declares its Iced window settings alongside the view, input, and context constructor it owns. The macro wires all of this into a single `Window` enum — `Window::Main`, `Window::Settings`, etc. — whose methods dispatch to the correct handlers at runtime.

## Adding a feature

1. Create a new module implementing the standard surface (`State`, `Message`, `Context`, `ContextMut`, `view`, `update`).
2. Add one line to the `register_features!` invocation.

That's it — dispatch, state initialization, and message routing are all generated.

## Architecture

See [`docs/`](./docs) for Architecture Decision Records covering state management, routing, async, theming, and localization.

## License

MIT
