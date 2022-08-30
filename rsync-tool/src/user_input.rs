use std::io::{self, stdin};
use strum::IntoEnumIterator;

use crate::{builder::Dir, User};

/// returns results from human input
pub fn prompt_extra_inputs() -> (User, User, u8, Vec<&'static str>) {
    let stdin = io::stdin();
    let mut user_as = String::new();
    let mut user_flags = vec![];
    // NOTE: user var
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
    // NOTE: flag var
    set_flag(&stdin, "Sync run ? y/n", "--delete", &mut user_flags);
    set_flag(&stdin, "Dry run ? y/n", "-n", &mut user_flags);
    println!("where do you want to backup");
    for dir in Dir::iter() {
        println!("{}: {:?}", dir as u8, dir);
    }
    stdin.read_line(&mut line).expect("can't read line");
    // NOTE: dir var
    let dir_index = line.trim().lines().last().unwrap().parse::<u8>().unwrap();
    (user_as, user_to, dir_index, user_flags)
}

fn set_flag(stdin: &io::Stdin, prompt: &str, flag: &'static str, args: &mut Vec<&str>) {
    let mut line = String::new();
    println!("{}", prompt);
    stdin.read_line(&mut line).expect("can't read line");
    match &line.trim().lines().last().unwrap().to_lowercase().as_str() {
        &"y" => args.push(flag),
        &"n" => {}
        _ => println!("invalid input"),
    }
}

pub fn build_custom_fmt(rev: bool) -> String {
    let stdin = stdin();
    let mut line = String::new();
    if rev {
        println!("you are using --rev to copy files from remote to local, specify the path in remote that you want to copy")
    }
    println!("enter your rsync destination:");
    stdin.read_line(&mut line).expect("can't read line");
    line.trim().to_string()
}
