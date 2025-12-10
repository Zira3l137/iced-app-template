## {{project-name}} Iced Template Workspace

This repository is a **cargo-generate template** for creating an Iced-based Rust application with a preconfigured workspace.
It is not an application itself, but a starting point you can clone into your own project.

### Features

- **Multi-crate workspace**: separates concerns into:
  - `{{project-name}}` – GUI application crate (Iced)
  - `{{project-name}}-core` – shared types, constants, logging, and `anyhow`-based error helpers
  - `{{project-name}}-cli` – optional CLI/entry helpers
- **Batteries-included app shell**:
  - Application/session state split (runtime vs persistent)
  - Window and feature routing driven by a `register_features!` macro
  - Theming hooks and a bundled Nerd Font for rich UI
- **Reusable widget library**:
  - Macros (`button!`, `frame!`, `nerd_text!`, `icon!`, `gradient!`)
  - Builder-pattern APIs in `app/widgets` for advanced customization
- **Targets Iced 0.14.0** with the new `iced::daemon` runtime setup
- **Documentation-first**:
  - `QUICKSTART.md` – orientation and basic workflow
  - `ARCHITECTURE.md` – how the application shell is structured
  - `ADDING_FEATURES.md` – how to add new windows/features
  - `WIDGETS.md` – widget and macro reference
  - `CONTRIBUTING.md` – contribution and style guidelines (if you publish your template)

### Using the Template with cargo-generate

```bash
cargo install cargo-generate

cargo generate \
  --git {{repository-url}} \
  --name my-iced-app
```

This will create a new workspace named `my-iced-app` based on this template.
From there you can:

```bash
cd my-iced-app
cargo run          # launch the GUI shell
```

Customize the generated project by:
- Adding or modifying features in `crates/{{project-name}}/src/app/features/`
- Adjusting application state, session, and theming in `crates/{{project-name}}/src/app/`
- Extending shared logic in `crates/{{project-name}}-core`

### When to Use This Template

Use this template if you want:
- A **structured starting point** for an Iced desktop application
- A **feature-based architecture** with clear patterns for adding windows/views
- A **consistent design system** via reusable widgets and themes

If you just need a minimal Iced example, the official Iced examples might be a better fit; this template is intended for applications that will grow over time.
