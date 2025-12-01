use super::windows::ApplicationWindow;
use crate::app::features::FeatureMessage;

#[derive(Debug, Clone)]
pub enum AppMessage {
    Window(WindowMessage),
    System(SystemMessage),
    Feature(FeatureMessage),
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    Close(iced::window::Id),
    Open(ApplicationWindow),
    InitializeMainWindow,
}

#[derive(Debug, Clone)]
pub enum SystemMessage {
    ExecuteCommand(String, Vec<String>),
    ExitApplication,
}

impl From<WindowMessage> for AppMessage {
    fn from(msg: WindowMessage) -> Self {
        AppMessage::Window(msg)
    }
}

impl From<SystemMessage> for AppMessage {
    fn from(msg: SystemMessage) -> Self {
        AppMessage::System(msg)
    }
}

impl From<FeatureMessage> for AppMessage {
    fn from(msg: FeatureMessage) -> Self {
        AppMessage::Feature(msg)
    }
}
