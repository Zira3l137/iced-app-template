use crate::app::{features::FeatureMessage, state::Window};
use iced::window::Id;

#[derive(Debug, Clone)]
pub enum Message {
    Feature(FeatureMessage),
    Window(WindowMessage),
    System(SystemMessage),
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    Invoke(Window),
    Hide(Id),
}

#[derive(Debug, Clone)]
pub enum SystemMessage {
    Exit,
}
