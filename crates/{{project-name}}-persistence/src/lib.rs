use std::{
    fs::{create_dir_all, read_to_string, write},
    path::Path,
    result::Result as StdResult,
};

use anyhow::{Context, Result};

pub trait Persistent {
    type State: serde::Serialize + serde::de::DeserializeOwned;

    fn write_state<P: AsRef<Path>>(path: P, state: &Self::State) -> Result<()> {
        let mut path = path.as_ref().to_path_buf();
        if path.is_dir() || !path.exists() {
            create_dir_all(&path).context("Failed to create session directory")?;
            path = path.join("state.toml");
        }

        let session_string =
            toml::to_string_pretty(state).context("Failed to serialize session")?;
        write(path, session_string)?;

        Ok(())
    }

    fn read_state<P: AsRef<Path>>(path: P) -> Option<Self::State> {
        let path = path.as_ref();
        if !path.exists() {
            tracing::warn!("Session file was not found");
            return None;
        }

        let Ok(session_json) = read_to_string(path) else {
            tracing::error!("Failed to read session file");
            return None;
        };

        let Ok(session): StdResult<Self::State, _> = toml::from_str(session_json.as_str()) else {
            tracing::error!("Failed to deserialize session");
            return None;
        };

        Some(session)
    }
}
