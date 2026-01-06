use crate::{
    app::features::{FeaturesState, main},
    app::session::PersistentState,
    macros::register_windows,
};

use iced::{
    Size, Theme,
    window::{Id, Settings},
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct State {
    pub windows: HashMap<Id, Window>,
    pub themes: HashMap<String, Theme>,
    pub features: FeaturesState,
    pub persistent: PersistentState,
}

register_windows!(
    Main : Settings {
        size: Size::new(800.0, 600.0),
        ..Default::default()
    } => main::view
);
