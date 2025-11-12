mod reader;
use color_eyre::Result;
use core::str;
use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, shade},
    widgets::{Block, Tabs, Gauge, Paragraph},
};

use ratatui::{DefaultTerminal, Frame};
use reader::Error;
use std::fs;
use std::{
    env,
    io::{self, Write},
    process::{Command, Output},
    vec,
};

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

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let gauge_ratio: &mut f64 = &mut 0.005;
    loop {
        terminal.draw(|x| render(x, gauge_ratio))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, gauge_ratio: &mut f64) {
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
    let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).block(cyan_block)
    .highlight_style(Style::default().cyan());
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
        .gauge_style(Style::new().italic())
        .ratio(*gauge_ratio)
        .label("")
        .use_unicode(true);
    let gauge_area = Rect::new(layout[1].x + 5, layout[1].y + 6, layout[1].width - 6, 1);
    frame.render_widget(gauge, gauge_area);
    set_gauge_ratio(gauge_ratio);
}

fn set_gauge_ratio(ratio: &mut f64) {
    let new_ratio = *ratio + 0.05;
    if new_ratio == 1.05 {
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
