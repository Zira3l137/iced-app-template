
```rust
// ============================================================
// Feature: <feature_name>
// ============================================================
// Submodule declarations — add as needed.
// mod submodule_a;
// mod submodule_b;

use crate::app::{
    App,
    message::{InputEvent, Message as GlobalMessage},
    state::FeatureMessage,
};

use iced::{Element, Task, keyboard, mouse, window::Id};

// ── Layout constants ─────────────────────────────────────────
// Keep these in sync with other features, or override per-feature.
pub(crate) const SEPARATOR_SIZE: f32 = 2.0;
pub(crate) const ICON_SIZE: u32 = 24;
pub(crate) const COL_PADDING: f32 = 10.0;
pub(crate) const COL_SPACING: f32 = 10.0;
pub(crate) const ROW_PADDING: f32 = 10.0;
pub(crate) const ROW_SPACING: f32 = 10.0;
pub(crate) const CONTAINER_PADDING: f32 = 10.0;

// ── State ────────────────────────────────────────────────────
// Feature-local runtime state. Add widget states and any
// ephemeral values that don't need to be persisted.
#[derive(Debug, Clone, Default)]
pub struct State {
    // example: search_state: text_input::State,
}

// ── Context (immutable) ──────────────────────────────────────
// Borrows from App for use in view(). Add fields as needed.
// Only include what view() actually reads.
#[derive(Debug, Clone)]
pub struct Context<'a> {
    feature_state: &'a State,
    // example: date: &'a DateTime<Local>,
    // example: locale: &'a Locale,
}

impl<'a> Context<'a> {
    pub fn new(app: &'a App) -> Self {
        Self {
            feature_state: &app.features_state.<feature_name>,
            // example: date: &app.app_state.date,
            // example: locale: &app.app_state.locales[&app.persistent_state.current_locale],
        }
    }
}

// ── ContextMut (mutable) ─────────────────────────────────────
// Mutable borrows from App for use in update(). Add fields
// for anything update() needs to mutate.
#[derive(Debug)]
pub struct ContextMut<'a> {
    feature_state: &'a mut State,
    // example: current_locale: &'a mut String,
}

impl<'a> ContextMut<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self {
            feature_state: &mut app.features_state.<feature_name>,
            // example: current_locale: &mut app.persistent_state.current_locale,
        }
    }
}

// ── Init ─────────────────────────────────────────────────────
// Called once by initialize_features(). Use to populate
// combo_box::State, caches, or any derived state that can't
// come from Default.
pub fn init(ctx: ContextMut<'_>) {
    let _ = ctx; // remove when init does real work
}

// ── Message ──────────────────────────────────────────────────
// Feature-local message variants. Each variant represents one
// user action or async result this feature handles.
#[derive(Debug, Clone)]
pub enum Message {
    // Example:
    // SomeAction(String),
    // ToggleSomething,
}

impl From<Message> for GlobalMessage {
    fn from(msg: Message) -> GlobalMessage {
        GlobalMessage::Feature(FeatureMessage::<FeatureVariant>(msg))
        // Replace <FeatureVariant> with the generated enum variant,
        // e.g. FeatureMessage::MyFeature(msg)
    }
}

// ── Update ───────────────────────────────────────────────────
// Handle feature messages. Return Task::none() for synchronous
// updates; return a Task for async effects or cross-feature
// messages.
pub fn update<'a>(msg: Message, ctx: ContextMut<'a>) -> Task<GlobalMessage> {
    match msg {
        // Example:
        // Message::SomeAction(value) => {
        //     // mutate ctx fields
        //     Task::none()
        // }
        // Message::ToggleSomething => {
        //     ctx.feature_state.some_flag = !ctx.feature_state.some_flag;
        //     Task::none()
        // }
    }
}

// ── View ─────────────────────────────────────────────────────
// Construct the feature's UI from an immutable Context.
// Delegate to submodules for logical sections.
pub fn view<'a>(ctx: Context<'a>, _window_id: Id) -> Element<'a, GlobalMessage> {
    // Example structure:
    //
    // let locale = ctx.locales.get(ctx.current_locale).expect("locale not found");
    //
    // let section_a = submodule_a::view(ctx.clone(), locale);
    // let section_b = submodule_b::view(ctx.clone(), locale);
    //
    // container(
    //     column![section_a, section_b]
    //         .spacing(COL_SPACING)
    //         .padding(COL_PADDING),
    // )
    // .align_top(Length::Fill)
    // .padding(CONTAINER_PADDING)
    // .into()

    todo!("implement view for <feature_name>")
}

// ── Input ────────────────────────────────────────────────────
// Handle raw input events forwarded from the window's
// subscription. Scope this to shortcuts and special drag
// behavior — normal widget input is handled by the widget tree.
pub fn input(input: &InputEvent) -> Task<GlobalMessage> {
    match input {
        InputEvent::Keyboard(keyboard) => match keyboard {
            keyboard::Event::KeyReleased { .. } => Task::none(),
            _ => Task::none(),
        },
        InputEvent::Mouse(mouse) => match mouse {
            mouse::Event::ButtonReleased(_) => Task::none(),
            _ => Task::none(),
        },
    }
}
```
