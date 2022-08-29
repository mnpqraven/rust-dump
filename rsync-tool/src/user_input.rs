use std::io::{self, stdin};
use strum::IntoEnumIterator;

use crate::{builder::Dir, User};

/// returns results from human input
pub fn prompt_extra_inputs() -> (User, User, u8, Vec<&'static str>) {
    let stdin = io::stdin();
    let mut user_as = String::new();

    println!("AUTH:");
    println!("user (as):");
    stdin.read_line(&mut user_as).expect("can't read line");
    let user_as = User::new(&user_as.trim());

    println!("user (to): (default: {})", user_as.name);
    let mut line = String::new();
    stdin.read_line(&mut line).expect("can't read line");
    let user_to = match line.trim().is_empty() {
        true => User::new(&user_as.name),
        false => User::new(&line.trim()),
    };
    // TODO: in main
    let mut preview_sync = vec![];
    println!("Sync run ? y/n");
    stdin.read_line(&mut line).expect("can't read line");
    match &line.trim().lines().last().unwrap().to_lowercase().as_str() {
        &"y" => preview_sync.push("--delete"),
        &"n" => {}
        _ => println!("invalid input"),
    }
    println!("Dry run ? y/n");
    stdin.read_line(&mut line).expect("can't read line");
    match &line.trim().lines().last().unwrap().to_lowercase().as_str() {
        &"y" => preview_sync.push("-n"),
        &"n" => {}
        _ => println!("invalid input"),
    }
    println!("where do you want to backup");
    for dir in Dir::iter() {
        println!("{}: {:?}", dir as u8, dir);
    }
    stdin.read_line(&mut line).expect("can't read line");
    // let dir_index_selected: u8 = line.trim().parse().unwrap();
    let dir_index = line.trim().lines().last().unwrap().parse::<u8>().unwrap();
    (user_as, user_to, dir_index, preview_sync)
}

pub fn build_custom_fmt() -> String {
    let stdin = stdin();
    let mut line = String::new();
    println!("enter your rsync destination:");
    stdin.read_line(&mut line).expect("can't read line");
    line.trim().to_string()
}
