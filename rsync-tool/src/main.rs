use rsync_tool::builder::{build_target_arg, HomeType};
use rsync_tool::tmp_worker::{clear_tmp_exclude, create_tmp_exclude};
use rsync_tool::user_input::prompt_extra_inputs;
pub mod ip_process;
pub mod tmp_worker;
use clap::Parser;
use rsync_tool::builder::Dir;
use rsync_tool::Nas;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};

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
    /// reverse data flow (copy data from remote address to local directory)
    #[clap(short, long = "rev", action, default_value_t = false)]
    reverse: bool,
}

// TODO: struct-ize user args ?
// NOTE: do we need Vec<User> validation with human input ?
fn main() -> Result<(), io::Error> {
    let args: Args = Args::parse();
    let use_volume = match args.use_var_services {
        true => HomeType::Volume1,
        _ => HomeType::VarServices,
    };

    // INFO: runtime
    let (user_as, user_to, dir_index, user_flags) = prompt_extra_inputs();

    let tmp = create_tmp_exclude()?;
    let ssh = vec![String::from("-e"), format!("ssh -p {}", &args.port)];
    println!("Browsing {} as {}", &user_to.name, &user_as.name);
    let use_dir = Dir::try_from(dir_index).unwrap();
    let mut remote = build_target_arg(
        user_as,
        Nas::connect(&args.host.to_string(), args.port),
        &use_dir,
        use_volume,
        user_to,
        args.reverse,
    );
    let path_pair: Vec<&String>;
    let current = String::from("./");
    if args.reverse {
        println!(
            "You are copying a file/directory from the remote machine to the local machine\nMake sure the given target argument is a path that exists in the remote machine"
        );
        // TODO: existence check
        match &use_dir {
            Dir::Custom => {}
            _ => {
                remote.push('/');
                remote.push_str(&args.local);
            }
        }
        path_pair = vec![&remote, &current];
    } else {
        path_pair = vec![&args.local, &remote];
    }
    println!("copying from {} to {}", &path_pair[0], &path_pair[1]);
    let mut output = Command::new("rsync")
        .arg("-avzx")
        // .arg("-e")
        // .arg(ssh)
        .args(ssh)
        .arg("--progress")
        .args(user_flags)
        .arg(format!("--exclude-from={}", tmp))
        .args(path_pair)
        .stdout(Stdio::piped())
        // INFO: debug
        // .stderr(Stdio::piped())
        .spawn()
        .expect(
            "can't run rsync command, do you have rsync installed ?\nare the file/directory paths correct ?",
        );
    let mut stdout_buf_stream = BufReader::new(output.stdout.as_mut().unwrap());
    let mut line = String::new();
    loop {
        match stdout_buf_stream.read_line(&mut line) {
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
