// INFO: created by following guide below:
// https://www.demainilpleut.dev/your-first-cli-tool-with-rust/
// importing lib
extern crate clap;

// which module of clap we're going to be using
use clap::{Arg, App};
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("kat")
        .version("0.0.1")
        .author("Othi / mnpqraven@gmail.com")
        .about("super simple UNIX's cat utility written in rust")
        .arg(Arg::with_name("file")
             .help("File to print")
             .empty_values(false)
            )
        .get_matches();
    if let Some(file) = matches.value_of("file") {
        println!("Value for file argument: {}", file);
        // ^ should print whatever you type
        // WARNING: need to check existence
        if Path::new(&file).exists() {
            println!("file {} exists!", file);
            // read line by line
            // NOTE: expect() consumes some, dumps out error str
            let mut f = File::open(file).expect("cannot open file");
            let mut data = String::new();
            f.read_to_string(&mut data).expect("cannot open file");
            println!("{}", data);
        } else {
            println!("no file with name {} found, did you make a typo ?", file);
                process::exit(1); //SIGOUT
        }
    }
}
