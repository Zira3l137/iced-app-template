mod features;
mod message;
mod session;
mod settings;
mod state;
mod theme;

use crate::utils::io::read_fonts;
use features::route_feature_update;
use message::{Message, SystemMessage, WindowMessage};
use session::{Session, load_session_to_state, read_session, save_state_to_session, write_session};
use settings::read_settings;
use state::{State, Window};
use theme::load_available_themes;

use std::path::PathBuf;

use anyhow::{Context, Result};
use iced::{
    Element, Font, Settings, Subscription, Task, Theme, daemon, event,
    widget::combo_box::State as ComboBoxState, window,
};

pub struct App {
    state: State,
    session: Session,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let mut state = State::default();
        let session = read_session("session.json").unwrap_or_default();

        load_session_to_state(&session, &mut state);
        load_available_themes(&mut state, "themes");

        // FIXME: Ugly
        state.features.main.theme_options =
            ComboBoxState::new(state.themes.keys().cloned().collect());

        (Self { state, session }, Task::done(Message::Window(WindowMessage::Invoke(Window::Main))))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Feature(feat_msg) => route_feature_update(&mut self.state, feat_msg),
            Message::System(sys_msg) => match sys_msg {
                SystemMessage::Exit => {
                    save_state_to_session(&self.state, &mut self.session);
                    if let Err(e) = write_session("session.json", &self.session) {
                        tracing::error!("Failed to write session: {}", e);
                    };
                    iced::exit()
                }
            },
            Message::Window(wnd_msg) => match wnd_msg {
                WindowMessage::Invoke(target_window) => {
                    let active_window = self.state.windows.values().find(|w| *w == &target_window);
                    if active_window.is_some() {
                        return Task::none();
                    }

                    let settings = target_window.settings();
                    let (id, task) = window::open(settings);
                    self.state.windows.insert(id, target_window);
                    task.discard()
                }

                WindowMessage::Hide(target_id) => {
                    if self.state.windows.remove(&target_id).is_some() {
                        window::close(target_id).chain(if self.state.windows.is_empty() {
                            Task::done(Message::System(SystemMessage::Exit))
                        } else {
                            Task::none()
                        })
                    } else {
                        Task::none()
                    }
                }
            },
        }
    }

    pub fn view<'a>(&'a self, id: window::Id) -> Element<'a, Message> {
        match self.state.windows.get(&id) {
            Some(window) => window.view(&self.state, &self.theme(id), id),
            None => unreachable!(),
        }
    }

    pub fn theme(&self, _: window::Id) -> Theme {
        self.state.themes.get(&self.state.persistent.current_theme).cloned().unwrap_or(Theme::Dark)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, window_id| match event {
            event::Event::Window(window::Event::Closed) => {
                Some(Message::Window(WindowMessage::Hide(window_id)))
            }
            _ => None,
        })
    }
}

pub fn run() -> Result<()> {
    let app_settings =
        read_settings("settings.toml").context("Failed to read application settings.")?;

    let fonts = read_fonts(PathBuf::from("resources").join("fonts"))
        .context("Failed to read application fonts.")?;

    let default_font_name = app_settings.default_font;
    let default_font = Font::with_name(Box::leak(default_font_name.into_boxed_str()));
    let settings = Settings { default_font, fonts, ..Default::default() };

    daemon(App::new, App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .settings(settings)
        .title(|app: &App, window_id: window::Id| {
            let window = app.state.windows.get(&window_id).map(|w| w.title()).unwrap_or("");
            format!("{} - {}", env!("CARGO_PKG_NAME"), window)
        })
        .run()
        .context("Failed to initialize application daemon.")
}
