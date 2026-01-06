use crate::app::{features::FeatureMessage, message::Message as GlobalMessage};

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(String),
}

impl From<Message> for GlobalMessage {
    fn from(msg: Message) -> GlobalMessage {
        GlobalMessage::Feature(FeatureMessage::Main(msg))
    }
}
