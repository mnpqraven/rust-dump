pub mod ip_process;
use clap::{Parser, ValueEnum};
use rpassword::read_password;
use std::{
    io::{self, stdout, Read, Write, BufRead},
    net::Ipv4Addr,
};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// local file/folder path to copy
    #[clap(value_parser)]
    target: String,

    /// target folder in nas
    #[clap(arg_enum, value_parser)]
    mode: Mode,

    #[clap(value_parser)]
    host: String,

    #[clap(short, value_parser, default_value_t = 22)]
    port: u16,

    /// sending data to remote or receiving data from remote
    #[clap(arg_enum, value_parser)]
    send_or_receive: Flow,

    /// dry run, rsync's -n flag
    preview: bool,

    /// with --delete flag or not
    sync: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum Flow {
    Send,
    Recv,
}

#[derive(Clone, Copy, ValueEnum)]
enum Mode {
    Local,
    Remote,
}

#[derive(Clone, Copy, ValueEnum)]
enum Dir {
    // /var/services/homes/othi = $HOME = /volume1/homes/othi
    Db1, // /volume1/db1
    // NOTE: volume1 changable in future ?
    NetBackup, // /volume1/NetBackup/othi
    // /var/services/NetBackup/othi
    Voice, // /var/services/homes/othi/music/voice
    // /volume1/homes/othi/music/voice
    Music, // /var/services/music
           // /volume1/music
}

// TODO: don't hardcode hostname
// TODO: progress bar
// TODO: exclude bin/node_modules etc. folders
fn main() -> Result<(), io::Error> {
    // let _args: Args = Args::parse();
    let _rando: Ipv4Addr = "127.0.0.1".parse().unwrap();

    // NOTE: io and user interaction
    println!("enter pw");
    // let mut buffer = String::new();
    // let stdin = io::stdin();
    // match stdin.read_line(&mut buffer) {
    //     Ok(_) => print!("{}", buffer),
    //     Err(error) => eprint!("{error}")
    // }
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    println!("The password is: '{}'", password);
    Ok(())
}
