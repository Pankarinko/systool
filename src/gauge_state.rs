use color_eyre::owo_colors::colors::Magenta;
use ratatui::style::{Color, Style};

pub struct GaugeState {
    pub progress: f64,
    pub rainbow_state: RainbowColor,
    pub bg: Color,
    pub fg: Color,
}

/*
The extra step through this enum is for enabling custom colors with relatively low effort
for terminals that support it.
 */
pub enum RainbowColor {
    Red(Color),
    Yellow(Color),
    Green(Color),
    Cyan(Color),
    Blue(Color),
    Magenta(Color),
}

impl GaugeState {
    pub fn advance_gauge(&mut self) {
        self.progress = (self.progress + 0.1).clamp(0.0, 100.0);
        if self.progress == 100.0 {
            self.progress = 0.0;
            match self.rainbow_state {
                RainbowColor::Red(color) => {
                    self.bg = color;
                    self.fg = Color::Rgb(126, 31, 134);
                    self.rainbow_state = RainbowColor::Yellow(self.fg);
                }
                RainbowColor::Yellow(color) => {
                    self.bg = color;
                    self.fg = Color::Rgb(161, 77, 160);
                    self.rainbow_state = RainbowColor::Green(self.fg);
                }
                RainbowColor::Green(color) => {
                    self.bg = color;
                    self.fg = Color::Rgb(157, 121, 188);
                    self.rainbow_state = RainbowColor::Cyan(self.fg);
                }
                RainbowColor::Cyan(color) => {
                    self.bg = color;
                    self.fg = Color::Rgb(140, 160, 215);
                    self.rainbow_state = RainbowColor::Blue(self.fg);
                }
                RainbowColor::Blue(color) => {
                    self.bg = color;
                    self.fg = Color::Rgb(145, 196, 242);
                    self.rainbow_state = RainbowColor::Magenta(self.fg);
                }
                RainbowColor::Magenta(color) => {
                    self.bg = color;
                    self.fg = Color::Rgb(150, 131, 236);
                    self.rainbow_state = RainbowColor::Red(self.fg);
                }
            }
        }
    }
}
