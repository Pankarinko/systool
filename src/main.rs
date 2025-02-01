use core::str;
use std::{io::{self, Write}, process::{Command, Output}};
use colored::Colorize;

fn main() {
  let kernel = Command::new("uname").arg("-r").output();
  match str::from_utf8(&kernel.unwrap().stdout) {
    Ok(version) => println!("{}", version.bright_red()),
    Err(_) => println!("{}", "No valid output!".red())
  }
  }
