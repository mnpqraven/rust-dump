use std::io::{self, stdin};

use clap::Parser;

fn main() -> Result<(), &'static str> {
    let default_ip = String::from("192.168.1.14");

    println!("ff or chrome ?\n1: ff\n2: chrome");
    // TODO: back to index
    println!("h: back to home");
    println!("Enter: select");
    // NOTE: synology api ?
    //
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    print!("{}", line);

    Ok(())
}
