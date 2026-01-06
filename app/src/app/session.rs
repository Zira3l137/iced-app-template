use crate::macros::register_session;
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::Path,
    result::Result as StdResult,
};

use anyhow::{Context, Result};
use serde_json::{from_str, to_string_pretty};

register_session!(
    (CurrentThemeField, current_theme, String) => "Currently selected theme used by the application"
);

pub fn write_session<P: AsRef<Path>>(path: P, session: &Session) -> Result<()> {
    let mut path = path.as_ref().to_path_buf();
    if path.is_dir() && !path.exists() {
        create_dir_all(&path).context("Failed to create session directory")?;
        path = path.join("session.json");
    }

    let session_string = to_string_pretty(session).context("Failed to serialize session")?;
    write(path, session_string)?;

    Ok(())
}

pub fn read_session<P: AsRef<Path>>(path: P) -> Option<Session> {
    let path = path.as_ref();
    if !path.exists() {
        tracing::warn!("Session file was not found");
        return None;
    }

    let Ok(session_json) = read_to_string(&path) else {
        tracing::error!("Failed to read session file");
        return None;
    };

    let Ok(session): StdResult<Session, _> = from_str(&session_json) else {
        tracing::error!("Failed to deserialize session");
        return None;
    };

    Some(session)
}
