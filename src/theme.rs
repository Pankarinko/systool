use ratatui::style::Color;

pub struct ColorTheme {
    pub main_left: Color,
    pub main_right: Color,
}

pub fn set_color_theme() -> ColorTheme {
    ColorTheme {
        main_left: Color::Rgb(161, 77, 160),
        main_right: Color::Rgb(145, 196, 242),
    }
}
