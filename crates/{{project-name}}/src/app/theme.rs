pub const DEFAULT_THEME: &str = "Light";

pub fn default_themes<'a>() -> [(&'a str, iced::Theme); 22] {
    [
        ("Light", iced::Theme::Light),
        ("Dark", iced::Theme::Dark),
        ("Dracula", iced::Theme::Dracula),
        ("Nord", iced::Theme::Nord),
        ("Solarized Light", iced::Theme::SolarizedLight),
        ("Solarized Dark", iced::Theme::SolarizedDark),
        ("Gruvbox Light", iced::Theme::GruvboxLight),
        ("Gruvbox Dark", iced::Theme::GruvboxDark),
        ("Catppuccin Latte", iced::Theme::CatppuccinLatte),
        ("Catppuccin Frappé", iced::Theme::CatppuccinFrappe),
        ("Catppuccin Macchiato", iced::Theme::CatppuccinMacchiato),
        ("Catppuccin Mocha", iced::Theme::CatppuccinMocha),
        ("Tokyo Night", iced::Theme::TokyoNight),
        ("Tokyo Night Storm", iced::Theme::TokyoNightStorm),
        ("Tokyo Night Light", iced::Theme::TokyoNightLight),
        ("Kanagawa Wave", iced::Theme::KanagawaWave),
        ("Kanagawa Dragon", iced::Theme::KanagawaDragon),
        ("Kanagawa Lotus", iced::Theme::KanagawaLotus),
        ("Moonfly", iced::Theme::Moonfly),
        ("Nightfly", iced::Theme::Nightfly),
        ("Oxocarbon", iced::Theme::Oxocarbon),
        ("Ferra", iced::Theme::Ferra),
    ]
}
