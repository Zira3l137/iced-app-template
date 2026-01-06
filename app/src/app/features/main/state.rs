use crate::app::state::State as GlobalState;

use iced::widget::combo_box::State as ComboBoxState;

#[derive(Debug, Clone, Default)]
pub struct State {
    pub theme_options: ComboBoxState<String>,
}

pub fn initialize(global_state: &mut GlobalState) {
    let themes = &global_state.themes;
    global_state.features.main.theme_options = ComboBoxState::new(themes.keys().cloned().collect());
}
