use crate::app::state::{FeatureMessage, Window};
use {{project-name}}_utils::command::Command;

use iced::{keyboard::Event as KeyboardEvent, mouse::Event as MouseEvent, window::Id};

#[derive(Debug, Clone)]
pub enum Message {
    App(AppMessage),
    System(SystemMessage),
    Feature(FeatureMessage),
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    View(Window),
    Hide(Id),
    Input(Id, InputEvent),
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

#[expect(unused)]
#[derive(Debug, Clone)]
pub enum SystemMessage {
    Execute(Command),
    Exit,
}
