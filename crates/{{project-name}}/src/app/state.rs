use iced::window::Id;

use {{project-name}}_core::types::Lookup;

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
    pub current_theme: String,
    pub themes: Lookup<String, iced::Theme>,
    pub windows: Lookup<Id, WindowInfo>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_theme: theme::DEFAULT_THEME.to_owned(),
            themes: theme::default_themes()
                .iter()
                .map(|(name, theme)| ((*name).to_owned(), theme.clone()))
                .collect(),
            windows: Default::default(),
        }
    }
}
