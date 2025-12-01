/// Macro to register all application features in one place.
/// This eliminates the need to manually update multiple files when adding a new feature.
///
/// # Usage
/// ```rust
/// register_features! {
///     // WindowVariant => module_name { width, height, position }
///     Root => root { 768.0, 768.0, Centered },
///     Options => settings { 768.0, 460.0, Centered },
///     NewFeature => new_feature { 600.0, 400.0, Centered },
/// }
/// ```
///
/// This single macro call will:
/// 1. Generate the FeatureMessage enum (replacing features/mod.rs boilerplate)
/// 2. Generate the FeaturesState struct (replacing state.rs boilerplate)
/// 3. Generate window configuration methods (replacing windows.rs match arms)
/// 4. Generate message routing (replacing mod.rs match arms)
#[macro_export]
macro_rules! register_features {
    (
        $(
            $variant:ident => $module:ident { $width:expr, $height:expr, $position:ident }
        ),+ $(,)?
    ) => {
        // 1. Generate FeatureMessage enum
        #[derive(Debug, Clone)]
        pub enum FeatureMessage {
            $(
                $variant($module::Message),
            )+
        }

        // 2. Generate FeaturesState struct
        #[derive(Debug, Default)]
        pub struct FeaturesState {
            $(
                pub $module: $module::State,
            )+
        }

        // 3. Generate window configuration methods
        impl super::windows::ApplicationWindow {
            pub fn name(&self) -> String {
                self.to_string()
            }

            pub fn default_size(&self) -> iced::Size {
                match self {
                    $(
                        Self::$variant => iced::Size { width: $width, height: $height },
                    )+
                }
            }

            pub fn default_position(&self) -> iced::window::Position {
                match self {
                    $(
                        Self::$variant => iced::window::Position::$position,
                    )+
                }
            }

            pub fn view<'a>(
                &self,
                app: &'a $crate::app::Application,
            ) -> iced::Element<'a, $crate::app::message::AppMessage> {
                match self {
                    $(
                        Self::$variant => $module::view(app)
                            .map(|m| $crate::app::message::AppMessage::Feature(FeatureMessage::$variant(m))),
                    )+
                }
            }
        }

        // 4. Generate message routing function
        pub fn route_feature_update(
            state: &mut FeaturesState,
            msg: FeatureMessage,
        ) -> $crate::app::AppTask {
            match msg {
                $(
                    FeatureMessage::$variant(msg) => $module::update(&mut state.$module, msg),
                )+
            }
        }
    };
}

/// Macro to declare window types with automatic trait implementations
///
/// Usage:
/// ```
/// declare_windows! {
///     Root,
///     Options,
///     NewFeature,
/// }
/// ```
#[macro_export]
macro_rules! declare_windows {
    ($($variant:ident),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Hash, Default, EnumString, EnumIter)]
        #[strum(serialize_all = "lowercase")]
        pub enum ApplicationWindow {
            #[default]
            $($variant,)+
        }

        impl ApplicationWindow {
            pub fn into_iter() -> impl Iterator<Item = Self> {
                Self::iter()
            }

            pub fn name(&self) -> String {
                self.to_string()
            }
        }
    };
}

/// Alternative: Generate only the routing logic, keeping manual control over structs/enums
///
/// Usage:
/// ```
/// impl_feature_routing! {
///     Root => root,
///     Settings => settings,
/// }
/// ```
#[macro_export]
macro_rules! impl_feature_routing {
    (
        $($variant:ident => $module:ident),+ $(,)?
    ) => {
        /// Route feature messages to their respective update handlers
        pub fn route_feature_update(
            state: &mut FeaturesState,
            msg: FeatureMessage,
        ) -> $crate::app::AppTask {
            match msg {
                $(
                    FeatureMessage::$variant(msg) => $module::update(&mut state.$module, msg),
                )+
            }
        }

        /// Route window views to their respective view functions
        pub fn route_feature_view<'a>(
            window: &super::windows::ApplicationWindow,
            app: &'a $crate::app::Application,
        ) -> Option<iced::Element<'a, $crate::app::message::AppMessage>> {
            match window {
                $(
                    super::windows::ApplicationWindow::$variant => Some(
                        $module::view(app).map(|m| $crate::app::message::AppMessage::Feature(FeatureMessage::$variant(m)))
                    ),
                )+
                _ => None,
            }
        }
    };
}

/// Generate window configuration methods without touching the enum definition
///
/// Usage:
/// ```
/// impl_window_configs! {
///     Root: { 768.0, 768.0, Centered },
///     Options: { 768.0, 460.0, Centered },
/// }
/// ```
#[macro_export]
macro_rules! impl_window_configs {
    (
        $($variant:ident: { $width:expr, $height:expr, $position:ident }),+ $(,)?
    ) => {
        impl ApplicationWindow {
            pub fn default_size(&self) -> iced::Size {
                match self {
                    $(
                        Self::$variant => iced::Size { width: $width, height: $height },
                    )+
                }
            }

            pub fn default_position(&self) -> iced::window::Position {
                match self {
                    $(
                        Self::$variant => iced::window::Position::$position,
                    )+
                }
            }
        }
    };
}
