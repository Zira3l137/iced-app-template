#![allow(dead_code)]

pub type Lookup<K, V> = hashbrown::HashMap<K, V, ahash::RandomState>;
pub type Entry<'a, K, V> = hashbrown::hash_map::Entry<'a, K, V, ahash::RandomState>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    #[default]
    SettingsAlt, // 
    Duplicate, // 
    Variable,  // 󱃻
    Settings,  // 
    Delete,    // 󰆴
    Cancel,    // 󰜺
    Import,    // 󰋺
    Export,    // 󰮓
    Apply,     // 
    Theme,     // 
    About,     // 
    Edit,      // 
    Copy,      // 󰆏
    Clear,     // 󰅘
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let codepoint = match self {
            Icon::Apply => '\u{f058}',
            Icon::Variable => '\u{f10fb}',
            Icon::Copy => '\u{f018f}',
            Icon::Edit => '\u{f044}',
            Icon::Duplicate => '\u{f4c4}',
            Icon::Delete => '\u{f01b4}',
            Icon::Cancel => '\u{f073a}',
            Icon::Settings => '\u{e690}',
            Icon::SettingsAlt => '\u{eb51}',
            Icon::Import => '\u{f02fa}',
            Icon::Export => '\u{f0b93}',
            Icon::Theme => '\u{e22b}',
            Icon::About => '\u{e66a}',
            Icon::Clear => '\u{f0158}',
        };
        write!(f, "{}", codepoint)
    }
}

#[macro_export]
macro_rules! lookup {
    [$($key: expr => $value: expr),*] => {
        {
            let mut lookup = $crate::core::types::Lookup::default();
            $(
                lookup.insert($key, $value);
            )*
            lookup
        }
    };
}
