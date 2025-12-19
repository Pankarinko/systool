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
                    self.fg = Color::Yellow;
                    self.rainbow_state = RainbowColor::Yellow(self.fg);
                }
                RainbowColor::Yellow(color) => {
                    self.bg = color;
                    self.fg = Color::Green;
                    self.rainbow_state = RainbowColor::Green(self.fg);
                }
                RainbowColor::Green(color) => {
                    self.bg = color;
                    self.fg = Color::Cyan;
                    self.rainbow_state = RainbowColor::Cyan(self.fg);
                }
                RainbowColor::Cyan(color) => {
                    self.bg = color;
                    self.fg = Color::Blue;
                    self.rainbow_state = RainbowColor::Blue(self.fg);
                }
                RainbowColor::Blue(color) => {
                    self.bg = color;
                    self.fg = Color::Magenta;
                    self.rainbow_state = RainbowColor::Magenta(self.fg);
                }
                RainbowColor::Magenta(color) => {
                    self.bg = color;
                    self.fg = Color::Red;
                    self.rainbow_state = RainbowColor::Red(self.fg);
                }
            }
        }
    }
}
