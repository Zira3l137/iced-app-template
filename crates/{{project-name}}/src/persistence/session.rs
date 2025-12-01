use serde_json;
use std::fs::write;

use {{crate_name}}_core::constants;

pub fn save(session: &crate::app::session::ApplicationSession) -> Result<(), std::io::Error> {
    let app_data_path = constants::local_app_data_path().join(constants::APP_TITLE);
    if !app_data_path.exists() {
        std::fs::create_dir_all(&app_data_path)?;
    }

    let session_string = serde_json::to_string_pretty(session)?;
    write(app_data_path.join("session.json"), session_string)?;

    Ok(())
}

pub fn load() -> Option<crate::app::session::ApplicationSession> {
    let app_data_path = constants::local_app_data_path().join(constants::APP_TITLE);
    if !app_data_path.exists() {
        return None;
    }

    let session_json = std::fs::read_to_string(app_data_path.join("session.json")).ok()?;
    let Ok(session): Result<crate::app::session::ApplicationSession, _> = serde_json::from_str(&session_json)
    else {
        return None;
    };

    Some(session)
}
