macro_rules! register_features {
    (
        $(
            $variant:ident => $module:ident
        ),+ $(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub enum FeatureMessage {
            $(
                $variant($crate::app::features::$module::message::Message),
            )+
        }

        #[derive(Debug, Clone, Default)]
        pub struct FeaturesState {
            $(
                pub $module :$crate::app::features::$module::state::State,
            )+
        }

        pub fn route_feature_update(
            state: &mut $crate::app::state::State,
            msg: FeatureMessage,
        ) -> iced::Task<$crate::app::message::Message> {
            match msg {
                $(
                    FeatureMessage::$variant(msg) => $module::update(state, msg),
                )+
            }
        }

        pub fn initialize_features(state: &mut $crate::app::state::State) {
            $(
                $module::state::initialize(state);
            )+
        }
    };
}

macro_rules! register_windows {
    (
        $(
            $window:ident : $settings:expr => $view:path
        ),+ $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Window {
            $(
                $window,
            )+
        }

        impl Window {
            pub fn title(&self) -> &str {
                match self {
                    $(
                        Window::$window => stringify!($window),
                    )+
                }
            }

            pub fn settings(&self) -> iced::window::Settings {
                match self {
                    $(
                        Window::$window => $settings,
                    )+
                }
            }

            pub fn view<'a>(&self, state: &'a $crate::app::state::State, theme: &iced::Theme, window_id: iced::window::Id) -> iced::Element<'a, $crate::app::message::Message> {
                match self {
                    $(
                        Window::$window => $view(state, theme, window_id),
                    )+
                }
            }
        }
    };
}

macro_rules! register_session {
    (
        $(
            ($field_struct:ident, $field_name:ident, $field_type:ty) => $description:literal
        ),+ $(,)?
    ) => {
        #[derive(Debug, Clone, Default)]
        pub struct PersistentState {
            $(
                #[doc = $description]
                pub $field_name: $field_type,
            )+
        }

        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct Session {
            $(
                #[doc = $description]
                $field_name: $field_type,
            )+
        }

        pub trait SessionField {
            fn load_from_session(session: &$crate::app::session::Session, state: &mut $crate::app::state::State);
            fn save_to_session(state: &$crate::app::state::State, session: &mut $crate::app::session::Session);
        }

        $(
            #[doc = $description]
            pub struct $field_struct;

            impl SessionField for $field_struct {
                fn load_from_session(session: &$crate::app::session::Session, state: &mut $crate::app::state::State) {
                    state.persistent.$field_name = session.$field_name.clone();
                }

                fn save_to_session(state: &$crate::app::state::State, session: &mut $crate::app::session::Session) {
                    session.$field_name = state.persistent.$field_name.clone();
                }
            }
        )+

        pub fn load_session_to_state(
            session: &crate::app::Session,
            state: &mut crate::app::state::State,
        ) {
            $(
                <$field_struct as crate::app::session::SessionField>::load_from_session(session, state);
            )+
        }

        pub fn save_state_to_session(
            state: &crate::app::state::State,
            session: &mut crate::app::session::Session,
        ) {
            $(
                <$field_struct as crate::app::session::SessionField>::save_to_session(state, session);
            )+
        }
    };

    () => {
        #[derive(Debug, Clone, Default)]
        pub struct PersistentState;

        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct Session;

        pub trait SessionField {
            fn load_from_session(session: &$crate::app::session::Session, state: &mut $crate::app::state::State);
            fn save_to_session(state: &$crate::app::state::State, session: &mut $crate::app::session::Session);
        }

        pub fn load_session_to_state(
            _session: &crate::app::Session,
            _state: &mut crate::app::state::State,
        ) {
            tracing::warn!("Attention, Session does not contain any fields to load!")
        }

        pub fn save_state_to_session(
            _state: &crate::app::state::State,
            _session: &mut crate::app::session::Session,
        ) {
            tracing::warn!("Attention, Session does not contain any fields to save!")
        }
    };
}

macro_rules! register_themes {
    (
        $(
            $theme_name:ident => $palette:expr
        ),+ $(,)?
    ) => {
        pub fn registered_themes() -> Vec<iced::Theme> {
            [
                $(iced::Theme::custom(stringify!($theme_name), $palette),)+
            ].to_vec()
        }
    };
}

pub(crate) use {register_features, register_session, register_themes, register_windows};
