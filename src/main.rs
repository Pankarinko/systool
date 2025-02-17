use core::str;
use std::{io::{self, Write}, process::{Command, Output}};
use colored::Colorize;

fn main() {
  /*Capture output from commands */
  let kernel = Command::new("uname").arg("-r").output();
  let user = Command::new("whoami").output();
  /* Output results */
  match str::from_utf8(&kernel.unwrap().stdout) {
    Ok(version) => { print!("kernel: ");
        println!("{}", version.bright_red())},
    Err(_) => println!("{}", "No valid output!".red())
  }
  match str::from_utf8(&user.unwrap().stdout) {
    Ok(user) => { print!("user: ");
    println!("{}", user.bright_blue())},
    Err(_) => println!("{}", "No valid output!".red()),
  }
  }
