use super::super::AppTask;
use super::super::message::{AppMessage, WindowMessage};
use super::super::windows::ApplicationWindow;
use super::FeatureMessage;
use crate::frame;

use iced::widget::column;
use iced::{Color, Length, alignment};

#[derive(Debug)]
pub struct State {
    // INFO: This struct holds the state of the feature.
}

impl Default for State {
    fn default() -> Self {
        Self {
            // INFO: Default state for the feature.
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    // INFO: This enum holds the messages that can be sent to the feature.
}

impl From<Message> for AppMessage {
    fn from(value: Message) -> Self {
        AppMessage::Feature(FeatureMessage::Root(value))
    }
}

pub fn update(state: &mut State, msg: Message) -> AppTask {
    match msg {
        // INFO: Handle the messages here.
    }
}

pub fn view<'a>(app: &'a crate::app::Application) -> iced::Element<'a, Message> {
    // INFO: Some essentials to manage the view based of state.
    let current_state = &app.state.features.root;
    let current_theme = app.theme();
    let palette = current_theme.palette();
    let palette_ext = current_theme.extended_palette();
    let bg_base_color = palette_ext.background.base.color;
    let bg_base_color_faded =
        Color::from_rgb(bg_base_color.r * 2.0, bg_base_color.g * 2.0, bg_base_color.b * 2.0);

    let root_col = column![
        // INFO: Add your content here.
    ];

    // INFO: Main content column gets enclosed in a frame here.
    frame!(root_col).padding(10).center_x(Length::Fill).align_top(Length::Fill).into()
}
