mod gauge_state;
mod reader;
use color_eyre::Result;
use core::str;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, bar::Set, shade},
    widgets::{Block, Gauge, Paragraph, Tabs},
};

use ratatui::{DefaultTerminal, Frame};
use reader::Error;
use std::{
    env,
    io::{self, Write},
    process::{Command, Output},
    vec,
};
use std::{fs, time::Duration};
type GaugeState = gauge_state::GaugeState;
type RainbowColor = gauge_state::RainbowColor;

static GAUGE_RATIO: f64 = 0.0;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    Style::default()
        .fg(Color::Black)
        .bg(Color::Green)
        .add_modifier(Modifier::ITALIC | Modifier::BOLD);
    let result = run(terminal);
    ratatui::restore();
    result
}

struct Settings {
    max_tabs: usize,
}

struct State {
    tab_num: usize,
    gauge_state: GaugeState,
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut state = State {
        tab_num: 0,
        gauge_state: GaugeState {
            progress: 0.0,
            rainbow_state: RainbowColor::Red(Color::Yellow),
            bg: Color::Red,
            fg: Color::Yellow,
        },
    };
    let settings = Settings { max_tabs: 4 };
    loop {
        state.gauge_state.advance_gauge();
        terminal.draw(|x| render(x, &state))?;
        let timeout = Duration::from_secs_f32(1.0 / 2000.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        break Ok(());
                    }
                    KeyCode::Tab => {
                        next_tab(&mut state, &settings);
                    }
                    KeyCode::BackTab => {
                        prev_tab(&mut state, &settings);
                    }
                    _ => (),
                }
            }
        }
    }
}

fn render(frame: &mut Frame, state: &State) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    let cyan_block = Block::bordered()
        .style(Style::new().bold())
        .border_style(Style::new().cyan().bold());
    let cyan_area = Rect::new(
        layout[0].x + 2,
        layout[0].y + 2,
        layout[0].width - 2,
        layout[0].height - 2,
    );

    /* Create Tabs with cyan border */
    let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
        .select(state.tab_num)
        .block(cyan_block)
        .style(Style::default().cyan())
        .highlight_style(Style::default().magenta());
    //let area = Rect::new(3, 3, (frame.area().width / 2) - 1, frame.area().height - 6);
    //frame.render_widget(cyan_block, cyan_area);
    frame.render_widget(tabs, cyan_area);
    let magenta_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(layout[1]);
    let magenta_block = Block::bordered()
        .style(Style::new().bold())
        .border_style(Style::new().magenta().bold());
    let magenta_area = Rect::new(
        layout[1].x + 2,
        layout[1].y + 2,
        layout[1].width - 2,
        layout[1].height - 2,
    );
    frame.render_widget(magenta_block, magenta_area);
    let title_area = Rect::new(layout[1].x + 5, layout[1].y + 4, layout[1].width - 5, 5);
    frame.render_widget(Paragraph::new("Title"), title_area);
    let gauge = Gauge::default()
        .block(Block::new())
        .gauge_style(
            Style::new()
                .italic()
                .fg(state.gauge_state.fg)
                .bg(state.gauge_state.bg),
        )
        .ratio(state.gauge_state.progress / 100.0)
        .label("")
        .use_unicode(true);
    let gauge_area = Rect::new(layout[1].x + 4, layout[1].y + 6, layout[1].width - 6, 1);
    frame.render_widget(gauge, gauge_area);
}

fn set_gauge_ratio(ratio: &mut f64) {
    let new_ratio = *ratio + 0.05;
    if new_ratio == 1.00 {
        *ratio = 0.05;
    } else {
        *ratio = new_ratio;
    }
}

fn read_data() {
    let names = ["User", "Device", "Kernel"];
    let mut values: Vec<String> = Vec::with_capacity(names.len());

    /*Capture output from commands */
    let device_name = fs::read_to_string("etc/hostname");
    let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .map_err(|_| Error)
        .and_then(|Output { stdout, .. }| String::from_utf8(stdout).map_err(|_| Error));
    match kernel {
        Ok(ker) => values.push(ker),
        Err(_) => println!("{}", ".... not found!!"),
    }
}

fn next_tab(state: &mut State, settings: &Settings) {
    state.tab_num = (state.tab_num + 1) % settings.max_tabs
}

fn prev_tab(state: &mut State, settings: &Settings) {
    state.tab_num = (settings.max_tabs + state.tab_num - 1) % settings.max_tabs
}
/*
let user = Command::new("").output();version
/* Output results */
match str::from_utf8(&user.stdout) {
  Ok(user) => values.push(user.to_string()),
  Err(_) => {println!("{}", "No valid output!".red());
  return},
}
match device_name {
  Ok(dev_name) => values.push(dev_name.to_string()),
  Err(_) => println!("{}", "Something went wrong :(".red()),
}
let mut val_iter = values.iter();
for name in names.iter() {
    let value = val_iter.next();
    if let Some(val) = value {
      println!("{name}:{val}")
  }
}*/
