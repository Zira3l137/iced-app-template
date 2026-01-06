pub mod main;

use crate::macros::register_features;

pub mod defaults {
    pub const DEFAULT_PADDING: f32 = 10.0;
    pub const DEFAULT_SPACING: f32 = 10.0;
}

register_features!(
    Main => main
);
