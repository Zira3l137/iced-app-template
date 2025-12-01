use iced::window::Id;

use {{crate_name}}_core::types::Lookup;

use super::features::FeaturesState;
use super::theme;
use super::windows::WindowInfo;

#[derive(Debug, Default)]
pub struct ApplicationState {
    pub ui: UiState,
    pub features: FeaturesState,
}

#[derive(Debug)]
pub struct UiState {
    pub themes: Lookup<String, iced::Theme>,
    pub windows: Lookup<Id, WindowInfo>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            themes: theme::default_themes()
                .iter()
                .map(|(name, theme)| ((*name).to_owned(), theme.clone()))
                .collect(),
            windows: Default::default(),
        }
    }
}
