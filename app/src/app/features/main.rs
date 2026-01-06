pub mod message;
pub mod state;

use super::defaults::{DEFAULT_PADDING, DEFAULT_SPACING};
use crate::app::{message::Message as GlobalMessage, state::State as GlobalState};
use message::Message as LocalMessage;

use iced::{
    Element, Length, Task, Theme,
    alignment::Horizontal,
    widget::{column, combo_box, container, iced, row, text},
    window::Id,
};

pub fn update(state: &mut GlobalState, msg: LocalMessage) -> Task<GlobalMessage> {
    match msg {
        LocalMessage::ThemeSelected(name) => {
            state.persistent.current_theme = name;
            Task::none()
        }
    }
}

pub fn view<'a>(
    global_state: &'a GlobalState,
    _theme: &Theme,
    _window_id: Id,
) -> Element<'a, GlobalMessage> {
    let local_state = &global_state.features.main;

    container(
        container(
            column![
                iced(24.0),
                combo_box(
                    &local_state.theme_options,
                    &global_state.persistent.current_theme,
                    Some(&global_state.persistent.current_theme),
                    |selection| LocalMessage::ThemeSelected(selection.clone()).into(),
                ),
                row![
                    column![
                        container(text("Primary"))
                            .style(container::primary)
                            .center(Length::Fill)
                            .padding(DEFAULT_PADDING),
                        container(text("Success"))
                            .style(container::success)
                            .center(Length::Fill)
                            .padding(DEFAULT_PADDING),
                    ]
                    .padding(DEFAULT_PADDING)
                    .spacing(DEFAULT_SPACING),
                    column![
                        container(text("Danger"))
                            .style(container::danger)
                            .center(Length::Fill)
                            .padding(DEFAULT_PADDING),
                        container(text("Warning"))
                            .style(container::warning)
                            .center(Length::Fill)
                            .padding(DEFAULT_PADDING),
                    ]
                    .padding(DEFAULT_PADDING)
                    .spacing(DEFAULT_SPACING),
                ]
                .padding(DEFAULT_PADDING)
                .spacing(DEFAULT_SPACING),
            ]
            .align_x(Horizontal::Center)
            .padding(DEFAULT_PADDING)
            .spacing(DEFAULT_SPACING),
        )
        .center(Length::Fill)
        .padding(DEFAULT_PADDING)
        .style(container::bordered_box),
    )
    .padding(DEFAULT_PADDING)
    .center(Length::Fill)
    .into()
}
