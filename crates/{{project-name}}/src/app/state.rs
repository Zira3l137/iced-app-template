use crate::{
    app::features::main,
    macros::{register_features, register_windows},
};

use {{crate_name}}_theme::load_available_themes;
use {{crate_name}}_utils::locale::Locale;
use iced::{
    Size, Theme,
    window::{Icon, Id, Settings},
};
use std::collections::HashMap;

const THEMES_PATH: &str = "themes";

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub icon: Option<Icon>,
    pub windows: HashMap<Id, Window>,
    pub themes: HashMap<String, Theme>,
    pub locales: HashMap<String, Locale>,
}

impl AppState {
    pub fn new(icon: Option<Icon>, locales: HashMap<String, Locale>) -> Self {
        Self { themes: load_available_themes(THEMES_PATH), icon, locales, ..Default::default() }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PersistentState {
    pub current_theme: String,
    pub current_locale: String,
}

register_features!(main::Main);

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
