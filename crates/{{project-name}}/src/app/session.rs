use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationSession {
    pub theme_selected: Option<String>,
}

impl std::default::Default for ApplicationSession {
    fn default() -> Self {
        Self { theme_selected: None }
    }
}
