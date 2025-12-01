#![allow(dead_code)]

use crate::error::Result;
use std::path::PathBuf;

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const APP_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const APP_TITLE: &str = "Env Inspector";

pub const APP_FONT_NAME: &str = "MonacoLigaturizedNerdFont.ttf";
pub const APP_FONT_FAMILY_NAME: &str = "MonacoLigaturized Nerd Font";

#[cfg(target_os = "windows")]
pub const EXPLORER_OPEN_PATH_COMMAND: &str = "explorer";

#[cfg(target_os = "linux")]
pub const EXPLORER_OPEN_PATH_COMMAND: &str = "xdg-open";

/// Application title + version.
pub fn app_title_full() -> String {
    format!("{APP_TITLE} v{APP_VERSION}")
}

/// Application title + version + authors + repository link.
pub fn app_info() -> String {
    format!("{APP_TITLE}\nVersion: {APP_VERSION}\nAuthors: {APP_AUTHORS}\nRepository: {APP_REPOSITORY}")
}

/// Returns the path to the application's data directory.
/// - On Windows, this will return `%LOCALAPPDATA%` environment variable.
/// - On Linux it will return `XDG_DATA_HOME` environment variable or `~/.local/share` if it is not set.
pub fn local_app_data_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or(String::from("")))
    }

    #[cfg(target_os = "linux")]
    {
        PathBuf::from(std::env::var("XDG_DATA_HOME").unwrap_or(String::from("~/.local/share")))
    }
}

pub fn resources_path() -> Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| crate::error::Error::other("Failed to get executable path", "resources_path"))?;

    Ok(exe_dir.join("resources").canonicalize()?)
}
