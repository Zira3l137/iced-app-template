use iced::widget::combo_box::State as ComboBoxState;

#[derive(Debug, Clone, Default)]
pub struct State {
    pub theme_options: ComboBoxState<String>,
}
