use rsync_tool::builder::build_target_arg;
use rsync_tool::builder::HomeType;
use strum::IntoEnumIterator;
pub mod ip_process;
pub mod tmp_worker;
use clap::Parser;
use clap::ValueEnum;
use rsync_tool::builder::Dir;
use rsync_tool::ip_process::find_ip;
use rsync_tool::tmp_worker::*;
use rsync_tool::Nas;
use rsync_tool::User;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

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
    mode: Option<Mode>,

    #[clap(short, value_parser, default_value_t = 22)]
    port: u16,
    // /// sending data to remote or receiving data from remote
    // #[clap(arg_enum, value_parser)]
    // receive: Option<bool>,
    #[clap(long, default_value_t = false)]
    use_volume: bool,
}

struct Interact {
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

// TODO: don't hardcode hostname
// TODO: NAS in lib integration
// TODO: dir enum
// NOTE: do we need Vec<User> validation with human input ?
fn main() -> Result<(), io::Error> {
    let args: Args = Args::parse();
    let mut preview_sync = vec![];
    let interact = Interact {
        preview: true,
        sync: false,
    };

    // TODO: data scraping
    let (ip, hostname) = find_ip(&args.host).unwrap();

    // INFO: runtime
    println!("TARGET: {}", &args.target);
    println!("IP:     {}", &ip);
    println!("HOST1:  {}", &args.host);
    println!("HOST2:  {}", &hostname.as_ref().unwrap());
    println!("PORT:   {}", &args.port);
    println!("preview {}", &interact.preview);
    println!("sync    {}", &interact.sync);

    // TODO: user input
    let stdin = io::stdin();
    let mut user_as = String::new();

    println!("AUTH:");
    println!("user (as):");
    stdin.read_line(&mut user_as)?;
    let user_as = User::new(&user_as.trim());

    println!("user (to): (default: {})", user_as.name);
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let user_to = match line.trim().is_empty() {
        true => User::new(&user_as.name),
        false => User::new(&line.trim()),
    };
    println!("Browsing {} as {}", &user_to.name, &user_as.name);
    // INFO: string builder
    // TODO: refactor
    println!("Sync run ? y/n");
    stdin.read_line(&mut line)?;
    match &line.trim().lines().last().unwrap().to_lowercase().as_str() {
        &"y" => preview_sync.push("--delete"),
        &"n" => {}
        _ => println!("invalid input"),
    }
    println!("Dry run ? y/n");
    stdin.read_line(&mut line)?;
    match &line.trim().lines().last().unwrap().to_lowercase().as_str() {
        &"y" => preview_sync.push("-n"),
        &"n" => {}
        _ => println!("invalid input"),
    }
    println!("where do you want to backup");
    for dir in Dir::iter() {
        println!("{}: {:?}", dir as u8, dir);
    }
    stdin.read_line(&mut line)?;
    // let dir_index_selected: u8 = line.trim().parse().unwrap();
    let dir_index = line.trim().lines().last().unwrap().parse::<u8>().unwrap();

    let tmp = create_tmp_exclude()?;
    let ssh = format!("ssh -p {}", &args.port);
    // NOTE: hardcode backup/debug
    // let arg_to = format!(
    //     "{}@{}:/volume1/NetBackup/{}",
    //     &user_as.name, &ip, &user_to.name
    // );
    let use_volume = match args.use_volume {
        true => HomeType::Volume,
        _ => HomeType::VarServices,
    };
    let yuge = build_target_arg(
        user_as,
        Nas::connect(&args.host.to_string(), args.port),
        Dir::try_from(dir_index).unwrap(),
        use_volume,
        user_to,
    );
    // NOTE: Debug
    // assert_eq!(yuge,arg_to);
    let mut output = Command::new("rsync")
        .arg("-avzx")
        .arg("-e")
        .arg(ssh)
        .arg("--progress")
        .args(preview_sync)
        .arg(format!("--exclude-from={}", tmp))
        .arg(&args.target)
        .arg(yuge)
        .stdout(Stdio::piped())
        // .stderr(Stdio::piped())
        .spawn()
        .expect("can't parse");
    let mut child_out = BufReader::new(output.stdout.as_mut().unwrap());
    let mut line = String::new();

    loop {
        match child_out.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                println!("{}", line.lines().last().unwrap());
                continue;
            }
            Err(err) => panic!("{}", err),
        }
    }

    clear_tmp_exclude()?;
    Ok(())
}
