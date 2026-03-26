use std::collections::HashMap;

use crate::app::{
    App,
    message::{InputEvent, Message as GlobalMessage},
    state::FeatureMessage,
};

use {{crate_name}}_utils::locale::Locale;
use iced::{
    Element, Length, Task, Theme, keyboard, mouse,
    theme::Base,
    widget::{button, column, container, iced, row, scrollable, text},
    window::Id,
};

pub const COL_PADDING: f32 = 10.0;
pub const COL_SPACING: f32 = 10.0;
pub const ROW_PADDING: f32 = 10.0;
pub const ROW_SPACING: f32 = 10.0;
pub const CONTAINER_PADDING: f32 = 10.0;

#[derive(Debug, Clone, Default)]
pub struct State {
    theme_menu_toggled: bool,
    locale_menu_toggled: bool,
}

#[derive(Debug, Clone)]
pub struct Context<'a> {
    feature_state: &'a State,
    current_theme: &'a str,
    current_locale: &'a str,
    themes: &'a HashMap<String, Theme>,
    locales: &'a HashMap<String, Locale>,
}

impl<'a> Context<'a> {
    pub fn new(app: &'a App) -> Self {
        Self {
            feature_state: &app.features_state.main,
            current_theme: &app.persistent_state.current_theme,
            current_locale: &app.persistent_state.current_locale,
            themes: &app.app_state.themes,
            locales: &app.app_state.locales,
        }
    }
}

#[derive(Debug)]
pub struct ContextMut<'a> {
    feature_state: &'a mut State,
    current_theme: &'a mut String,
    current_locale: &'a mut String,
    themes: &'a mut HashMap<String, Theme>,
    locales: &'a mut HashMap<String, Locale>,
}

impl<'a> ContextMut<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self {
            feature_state: &mut app.features_state.main,
            current_theme: &mut app.persistent_state.current_theme,
            current_locale: &mut app.persistent_state.current_locale,
            themes: &mut app.app_state.themes,
            locales: &mut app.app_state.locales,
        }
    }
}

pub fn init(_ctx: ContextMut<'_>) {}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeMenuToggle,
    LocaleMenuToggle,
    ThemeSwitch(String),
    LocaleSwitch(String),
}

impl From<Message> for GlobalMessage {
    fn from(msg: Message) -> GlobalMessage {
        GlobalMessage::Feature(FeatureMessage::Main(msg))
    }
}

pub fn update<'a>(msg: Message, ctx: ContextMut<'a>) -> Task<GlobalMessage> {
    match msg {
        Message::ThemeSwitch(theme_name) => {
            *ctx.current_theme = theme_name;
            Task::done(Message::ThemeMenuToggle.into())
        }
        Message::LocaleSwitch(locale_tag) => {
            *ctx.current_locale = locale_tag;

            Task::done(Message::LocaleMenuToggle.into())
        }
        Message::LocaleMenuToggle => {
            let previous_state = ctx.feature_state.locale_menu_toggled;
            ctx.feature_state.locale_menu_toggled = !previous_state;

            Task::none()
        }
        Message::ThemeMenuToggle => {
            let previous_state = ctx.feature_state.theme_menu_toggled;
            ctx.feature_state.theme_menu_toggled = !previous_state;

            Task::none()
        }
    }
}

pub fn view<'a>(ctx: Context<'a>, _window_id: Id) -> Element<'a, GlobalMessage> {
    let current_locale = ctx.current_locale;
    let current_theme = ctx.current_theme;

    let theme = ctx.themes.get(current_theme).expect("theme not found");
    let locale = ctx.locales.get(current_locale).expect("locale not found");

    let current_locale_tag = locale.as_tag();
    let theme_name = theme.name().to_owned();
    let get_string = |key: &str| locale.get_string("main", key);

    let locale_menu = container(scrollable(ctx.locales.values().map(|l| l.as_tag()).fold(
        column![].padding(COL_PADDING),
        |col, tag| {
            col.push(
                button(text(tag.clone()).width(Length::Fill).center())
                    .width(Length::Fill)
                    .on_press_maybe(
                        (tag != current_locale_tag)
                            .then_some(Message::LocaleSwitch(tag.clone()).into()),
                    ),
            )
        },
    )))
    .padding(CONTAINER_PADDING);

    let theme_menu = container(scrollable(ctx.themes.iter().fold(
        column![].padding(COL_PADDING),
        |col, (name, theme)| {
            col.push(
                button(text(name).width(Length::Fill).center())
                    .width(Length::Fill)
                    .on_press_maybe(
                        (*name != theme_name).then_some(Message::ThemeSwitch(name.clone()).into()),
                    )
                    .style(|_, status| {
                        let extended = theme.extended_palette();
                        let base_color = extended.primary.base.color;
                        let text_color = extended.primary.base.text;

                        match status {
                            button::Status::Active | button::Status::Pressed => button::Style {
                                background: Some(base_color.into()),
                                text_color,
                                ..Default::default()
                            },
                            button::Status::Hovered => button::Style {
                                background: Some(base_color.scale_alpha(0.8).into()),
                                text_color: text_color.scale_alpha(0.8),
                                ..Default::default()
                            },
                            button::Status::Disabled => button::Style {
                                background: Some(base_color.scale_alpha(0.5).into()),
                                text_color: text_color.scale_alpha(0.5),
                                ..Default::default()
                            },
                        }
                    }),
            )
        },
    )))
    .padding(CONTAINER_PADDING);

    let theme_menu_toggle = button(text(theme_name)).on_press_maybe(
        (!ctx.feature_state.theme_menu_toggled).then_some(Message::ThemeMenuToggle.into()),
    );

    let locale_menu_toggle = button(text(locale.language.clone())).on_press_maybe(
        (!ctx.feature_state.locale_menu_toggled).then_some(Message::LocaleMenuToggle.into()),
    );

    let theme_switcher: Element<'a, GlobalMessage> = if ctx.feature_state.theme_menu_toggled {
        theme_menu.into()
    } else {
        theme_menu_toggle.into()
    };

    let locale_switcher: Element<'a, GlobalMessage> = if ctx.feature_state.locale_menu_toggled {
        locale_menu.into()
    } else {
        locale_menu_toggle.into()
    };


    let theme_switch_area = column![text(get_string("theme_label")), theme_switcher];
    let locale_switch_area = column![text(get_string("locale_label")), locale_switcher];

    let control_row =
        row![theme_switch_area, locale_switch_area].padding(ROW_PADDING).spacing(ROW_SPACING);

    container(column![iced(22), control_row].spacing(COL_SPACING).padding(COL_PADDING))
        .center(Length::Fill)
        .padding(CONTAINER_PADDING)
        .into()
}

pub fn input(input: &InputEvent) -> Task<GlobalMessage> {
    match input {
        InputEvent::Keyboard(keyboard) => match keyboard {
            keyboard::Event::KeyReleased { key, location, modifiers, .. } => {
                tracing::info!(
                    "Key released: {:?}, location: {:?}, modifiers: {:?}",
                    key,
                    location,
                    modifiers
                );
                Task::none()
            }
            _ => Task::none(),
        },
        InputEvent::Mouse(mouse) => match mouse {
            mouse::Event::ButtonReleased(btn) => {
                tracing::info!("Mouse button released: {:?}", btn);
                Task::none()
            }
            _ => Task::none(),
        },
    }
}
