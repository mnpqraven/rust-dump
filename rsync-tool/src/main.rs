use rsync_tool::builder::build_target_arg;
use rsync_tool::builder::HomeType;
use rsync_tool::user_input::prompt_extra_inputs;
pub mod ip_process;
pub mod tmp_worker;
use clap::Parser;
use clap::ValueEnum;
use rsync_tool::builder::Dir;
use rsync_tool::tmp_worker::*;
use rsync_tool::Nas;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// local file/folder path to copy
    #[clap(value_parser)]
    local: String,
    /// nas address
    #[clap(value_parser)]
    host: String,
    /// ssh port
    #[clap(short, value_parser, default_value_t = 22)]
    port: u16,
    /// use /var/services directory tree instead of /volume1
    #[clap(long = "var", default_value_t = false)]
    use_var_services: bool,
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

// TODO: NAS in lib integration
// TODO: dir enum
// NOTE: do we need Vec<User> validation with human input ?
fn main() -> Result<(), io::Error> {
    let args: Args = Args::parse();
    let use_volume = match args.use_var_services {
        true => HomeType::Volume1,
        _ => HomeType::VarServices,
    };

    // INFO: runtime
    let (user_as, user_to, dir_index, preview_sync) = prompt_extra_inputs();

    let tmp = create_tmp_exclude()?;
    let ssh = format!("ssh -p {}", &args.port);
    println!("Browsing {} as {}", &user_to.name, &user_as.name);
    let remote = build_target_arg(
        user_as,
        Nas::connect(&args.host.to_string(), args.port),
        Dir::try_from(dir_index).unwrap(),
        use_volume,
        user_to,
    );
    dbg!(&remote);
    let mut output = Command::new("rsync")
        .arg("-avzx")
        .arg("-e")
        .arg(ssh)
        .arg("--progress")
        .args(preview_sync)
        .arg(format!("--exclude-from={}", tmp))
        .arg(&args.local)
        .arg(&remote)
        .stdout(Stdio::piped())
        // .stderr(Stdio::piped())
        .spawn()
        .expect("can't run rsync command, do you have rsync installed ?");
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
