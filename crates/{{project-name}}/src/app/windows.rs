use std::path::PathBuf;

use iced::Task;
use strum::{Display, EnumIter, EnumString};

use {{crate_name}}_core::constants;

use crate::app::AppTask;
use crate::app::message;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct WindowInfo {
    pub window_type: ApplicationWindow,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Hash, Default, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum ApplicationWindow {
    #[default]
    Root,
}

pub fn save_current_session(session: &crate::app::session::ApplicationSession) -> AppTask {
    if let Err(e) = crate::persistence::session::save(session) {
        tracing::error!("Failed saving session: {e}");
    }
    Task::none()
}

pub fn exit_application(
    session: &mut crate::app::session::ApplicationSession,
    state: &mut crate::app::state::ApplicationState,
) -> AppTask {
    let mut main_window_closed = true;
    let mut all_closed = true;

    state.ui.windows.iter().for_each(|(_, wnd_info)| {
        if !wnd_info.is_closed {
            if let ApplicationWindow::Root = wnd_info.window_type {
                main_window_closed = false;
            }
            all_closed = false;
        }
    });

    if all_closed || main_window_closed {
        tracing::info!("Exiting application");
        return save_current_session(session).chain(iced::exit());
    }

    iced::Task::none()
}

pub fn close_window(state: &mut crate::app::state::ApplicationState, wnd_id: &iced::window::Id) -> AppTask {
    if let Some(wnd_info) = state.ui.windows.get_mut(wnd_id) {
        tracing::info!("Closing window: {}", wnd_info.window_type);
        wnd_info.is_closed = true;
    }

    iced::Task::chain(
        iced::window::close(*wnd_id),
        Task::done(message::SystemMessage::ExitApplication.into()),
    )
}

pub fn invoke_window(state: &mut crate::app::state::ApplicationState, window: &ApplicationWindow) -> AppTask {
    let icon_path = match constants::resources_path() {
        Ok(path) => path.join("icon.ico"),
        Err(e) => {
            tracing::warn!("Failed to get resources path: {e}");
            PathBuf::from("resources/icon.ico").canonicalize().unwrap()
        }
    };
    let mut icon = iced::window::icon::from_file(icon_path);
    icon = icon.inspect_err(|e| tracing::warn!("Failed to load icon: {e}"));

    let (id, task) = iced::window::open(iced::window::Settings {
        position: window.default_position(),
        size: window.default_size(),
        icon: icon.ok(),
        exit_on_close_request: false,
        ..Default::default()
    });

    tracing::info!("Opening window: {}", window.name());
    state.ui.windows.insert(id, WindowInfo { window_type: *window, is_closed: false });
    task.then(|_| Task::none())
}
