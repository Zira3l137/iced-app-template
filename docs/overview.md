# Iced Desktop App Template — Architecture Notes

## Overview

A Rust desktop application template built on Iced using the daemon model. The core goal is a scalable, macro-friendly architecture where adding a new feature requires minimal manual changes across the codebase.

---

## State Structure

The app splits state into three domains:

```
App
├── AppState        — runtime only (windows, locale, fonts)
├── PersistentState — serialized to disk via Persistent trait
└── FeaturesState   — per-feature State structs (g::State, h::State, …)
```

`FeaturesState` holds named fields for each feature's own state. Features are initialized from `AppState` and `PersistentState` via `initialize_features()`.

---

## Feature Module Shape

Every feature module exposes the same surface:

```
mod g
├── State           — feature-owned, derives Debug + Default
├── Message         — feature-specific variants
├── Context<'a>     — immutable borrows from App, used in view()
├── ContextMut<'a>  — mutable borrows from App, used in update()
├── view()          — (Context, window_id) → Element
└── update()        — (Message, ContextMut) → Task<Message>
```

`Context` and `ContextMut` are constructed via `::new(&App)` / `::new(&mut App)`. The user defines which fields each context borrows — feature's own state may be included or omitted depending on need.

---

## Message Flow

```
OS event / window close
    ↓
subscription()
    ↓
Message  ──→  App / System / Feature
    ↓
App::update()
    ├── Message::App     — handled inline
    ├── Message::System  — Input dispatch to Window::input()
    │                      HideWindow, etc.
    └── Message::Feature → route_feature_message()
                               → f::update(msg, f::ContextMut::new(app))
                               → Task<Message>  (→ next update cycle)
```

Cross-feature effects are expressed as returned `Task`s, not direct calls. Features never call each other synchronously.

---

## Borrow Safety

`ContextMut::new(&mut App)` takes exclusive access to all of `App`, but this is safe because:

- `route_feature_message` dispatches via `match` — arms are mutually exclusive
- Only one `ContextMut` is alive at any point
- No two features need to be updated simultaneously in the same cycle

This also enables the macro to generate dispatch uniformly without knowing which fields each context borrows.

---

## Macro Design Intent

The four generated items:

```rust
register_features!(
    G => g,
    H => h,
    I => i,
);
```

Generates:

- `FeaturesState` — struct with `g: g::State, h: h::State, …`
- `FeatureMessage` — enum with `G(g::Message), H(h::Message), …`
- `initialize_features()` — calls each feature's initializer
- `route_feature_message()` — dispatches to `f::update(msg, f::ContextMut::new(app))`

Adding a new feature = one line in the macro invocation.

---

## Input Handling

Raw input is captured globally in `subscription()` and routed as messages:

```
Mouse / Keyboard event
    → SystemMessage::Input(window_id, InputEvent)
    → App::update matches window_id
    → Window::input(&InputEvent) → Task<Message>
```

Window close requests route to `SystemMessage::HideWindow(id)`.

---

## Entry Point

File paths are hardcoded as constants. Config, locales, fonts, and icon are all loaded at runtime before the daemon starts.

```rust
const SETTINGS_PATH: &str = "…";
const LOCALES_PATH:  &str = "…";
const FONTS_PATH:    &str = "…";
const ICON_PATH:     &str = "…";
```

Loading order:

```
read_config(SETTINGS_PATH)   — parses TOML, yields Settings (e.g. default_font name)
read_locales(LOCALES_PATH)   — yields Vec<Locale>
read_fonts(FONTS_PATH)       — yields Vec<Cow<'static, [u8]>>
read_icon(ICON_PATH)         — yields Option<Icon>
```

Guards:
- Abort if no locales found (logged via `tracing::error!`)
- `Box::leak` used for the default font name to satisfy `iced::Font::with_name`'s `'static` requirement

---

## Key Decisions

| Decision | Rationale |
|---|---|
| `ContextMut::new(&mut App)` over pre-split refs | Enables uniform macro dispatch |
| Cross-feature effects via `Task` | Keeps match arms independent; no aliasing issue |
| Per-feature `State` structs in `FeaturesState` | Clear ownership; each feature defaults independently |
| Input routed through `SystemMessage` | Centralizes event handling; avoids `Window` producing raw tasks |
| Daemon model (not `application`) | Required for multi-window support |
| Hardcoded config paths as constants | Pragmatic; template user places them where they prefer |

---

## Open Items

- `Window::input()` returns `Task<Message>` but most Iced widget input is handled internally by the widget tree — scope this to keyboard shortcuts / special drag behavior only
- `Persistent::read_state()` has no path parameter yet — implementation will need a strategy (constant path, env var, or passed in from `main`)
- `read_settings` to be renamed `read_config`
