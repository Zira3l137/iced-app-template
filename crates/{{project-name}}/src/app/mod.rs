pub mod features;
pub mod macros;
pub mod message;
pub mod session;
pub mod state;
pub mod theme;
pub mod widgets;
pub mod windows;

use {{crate_name}}_core::{constants, error::Error, types::Lookup};
use features::route_feature_update;

use crate::persistence;

pub type AppTask = iced::Task<message::AppMessage>;
pub type AppElement<'a> = iced::Element<'a, message::AppMessage>;

#[derive(Debug, Default)]
pub struct Application {
    pub session: session::ApplicationSession,
    pub state: state::ApplicationState,
}

impl Application {
    fn init_session(state: &mut state::ApplicationState, session: &session::ApplicationSession) {
        if let Some(theme) = &session.theme_selected {
            // INFO: Manage your session here
        }
    }

    pub fn new() -> (Self, AppTask) {
        let mut state = state::ApplicationState::default();
        let session = persistence::session::load().unwrap_or_default();

        Self::init_session(&mut state, &session);

        let app = Self { session, state };
        let _ = app.load_font().map(|result| match result {
            Ok(_) => return,
            Err(e) => tracing::error!("{e}"),
        });

        (app, iced::Task::done(message::AppMessage::Window(message::WindowMessage::InitializeMainWindow)))
    }

    fn load_font(&self) -> iced::Task<Result<(), Error>> {
        use std::path::PathBuf;

        let font_path = match constants::resources_path() {
            Ok(path) => path.join(constants::APP_FONT_NAME),
            Err(e) => {
                tracing::warn!("Failed to get resources path: {e}");
                PathBuf::from(format!("resources/{}", constants::APP_FONT_NAME)).canonicalize().unwrap()
            }
        };

        match std::fs::read(&font_path) {
            Ok(bytes) => iced::font::load(bytes)
                .map(|o| o.map_err(|_| Error::other("Failed to load font", "Application::load_font"))),
            Err(e) => iced::Task::done(Err(e.into())),
        }
    }

    pub fn update(&mut self, message: message::AppMessage) -> AppTask {
        match message {
            message::AppMessage::Window(msg) => match msg {
                message::WindowMessage::Close(wnd_id) => windows::close_window(&mut self.state, &wnd_id),
                message::WindowMessage::Open(window) => {
                    let open_windows = self
                        .state
                        .ui
                        .windows
                        .iter()
                        .filter_map(|(id, info)| (!info.is_closed).then_some((info.window_type, *id)))
                        .collect::<Lookup<_, _>>();

                    if let Some(open_window_id) = open_windows.get(&window) {
                        return iced::Task::done(message::WindowMessage::Close(*open_window_id).into());
                    }

                    windows::invoke_window(&mut self.state, &window)
                }
                message::WindowMessage::InitializeMainWindow => windows::invoke_window(&mut self.state, windows::ApplicationWindow::Root),
            },

            message::AppMessage::System(msg) => match msg {
                message::SystemMessage::ExecuteCommand(cmd, args) => {
                    tracing::info!("Executing command: {cmd} {}", args.join(" "));
                    let cmd_args = args.iter().map(String::as_str).collect::<Vec<_>>();
                    if let Err(err) = crate::platform::commands::execute_cmd(&cmd, &cmd_args) {
                        tracing::error!("Error executing command: {err}");
                    }
                    iced::Task::none()
                }
                message::SystemMessage::ExitApplication => {
                    windows::exit_application(&mut self.session, &mut self.state)
                }
            },

            message::AppMessage::Feature(msg) => route_feature_update(&mut self.state.features, msg),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<message::AppMessage> {
        iced::event::listen_with(|event, _, id| match event {
            iced::Event::Window(iced::window::Event::CloseRequested) => {
                Some(message::AppMessage::Window(message::WindowMessage::Close(id)))
            }
            _ => None,
        })
    }

    pub fn theme(&self) -> iced::Theme {
        let theme_name = &self.state.features.settings.current_theme;
        self.state.ui.themes.get(theme_name).cloned().unwrap_or_else(|| {
            if let Some(theme) = &self.session.theme_selected {
                tracing::warn!("Theme {theme} not found, defaulting to dark");
            }
            iced::Theme::Dark
        })
    }

    pub fn view(&self, id: iced::window::Id) -> AppElement {
        if let Some((_, wnd_state)) = self.state.ui.windows.iter().find(|(wnd_id, _)| **wnd_id == id) {
            wnd_state.window_type.view(self)
        } else {
            iced::widget::container(iced::widget::text("Window not found")).into()
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    iced::daemon(constants::APP_TITLE, Application::update, Application::view)
        .theme(|state, _| Application::theme(state))
        .subscription(Application::subscription)
        .run_with(move || Application::new())?;
    Ok(())
}
