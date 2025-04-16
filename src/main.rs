mod reader;
use colored::Colorize;
use core::str;
use ratatui;
use reader::Error;
use std::fs;
use std::{
    env,
    io::{self, Write},
    process::{Command, Output},
    vec,
};

fn main() {
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
        Err(_) => println!("{}", ".... not found!!".red()),
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
