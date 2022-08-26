pub mod ip_process;
use clap::{Parser, ValueEnum};
use rsync_tool::Nas;
use std::io;

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
    Db1,
    NetBackup,
    Voice,
    Music,
}

// TODO: don't hardcode hostname
// TODO: progress bar
// TODO: exclude bin/node_modules etc. folders
fn main() -> Result<(), io::Error> {
    // let _args: Args = Args::parse();

    // NOTE: io and user interaction
    // let mut buffer = String::new();
    // let stdin = io::stdin();
    // match stdin.read_line(&mut buffer) {
    //     Ok(_) => print!("{}", buffer),
    //     Err(error) => eprint!("{error}")
    // }
    // std::io::stdout().flush().unwrap();
    // let password = read_password().unwrap();
    // println!("The password is: '{}'", password);

    Nas::connect("192.168.1.14", 6661);
    Ok(())
}
