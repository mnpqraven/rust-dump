pub mod ip_process;
use clap::{Parser, ValueEnum};
use std::{io, process::Command};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// local file/folder path to copy
    #[clap(long, short, value_parser)]
    target: String,

    /// nas address
    #[clap(long, short, value_parser)]
    host: String,

    /// target folder in nas
    #[clap(arg_enum, value_parser)]
    mode: Mode,

    #[clap(short, value_parser, default_value_t = 22)]
    port: u16,

    /// sending data to remote or receiving data from remote
    #[clap(arg_enum, value_parser)]
    receive: Option<bool>,

    /// dry run, rsync's -n flag
    preview: Option<bool>,

    /// with --delete flag or not
    sync: Option<bool>,
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
    let args: Args = Args::parse();

    let mut preview = String::new();
    match args.preview {
        Some(_) => preview.push_str("-n"),
        _ => (),
    };
    match args.sync {
        Some(_) => {}
        None => {}
    };
    let me = Command::new("rsync")
        .args(["-e", "ssh", "-p", &args.port.to_string() ])
        .arg(preview)
        .output()
        .expect("failed to run rsync");
    Ok(())
}
