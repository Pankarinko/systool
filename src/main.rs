mod reader;
use color_eyre::Result;
use colored::Colorize;
use core::str;
use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders},
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
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
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
    // let area = Rect::new(3, 3, (frame.area().width / 2) - 1, frame.area().height - 6);
    frame.render_widget(cyan_block, cyan_area);
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
