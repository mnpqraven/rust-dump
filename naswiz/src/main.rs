use clap::{App, Arg};
use std::{
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

use naswiz::check_ip;
use naswiz::file_inject;
use naswiz::what_replace;
use naswiz::NAS;

fn main() -> Result<(), &'static str> {
    // hardcode for now
    let host = "othiremote.synology.me";
    let mountpoint = "/etc/systemd/system/media-nasremote-music.mount";

    // NOTE: meta
    let app = App::new("naswiz")
        .version("0.1.1")
        .author("othi")
        .about("nas mountpoint update wizard")
        .arg(
            Arg::new("ip")
                .help("nas' public ip address")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(true),
        )
        .get_matches();

    // NOTE: RUNTIME
    if let Some(ip) = app.value_of("ip") {
        let nas = NAS::new(
            host.to_string(),
            ip.to_string(),
            File::open(mountpoint)
            .expect("can't find said file\nDoes the file exist ?/Do you have permission to view the file ?"),
        );
        match check_ip(&nas) {
            Ok(_) => {
                println!("{} is a valid ip address, continuing ...", &nas.ip);
                let dump = what_replace(mountpoint.to_string(), nas.ip).to_string();
                file_inject(dump).expect("can't write to file");

                // should eventually pass paths and file name as args
                copy_to_systemd();
            }
            Err(_) => panic!("not a valid ip address"),
        }
    }
    Ok(())
}
fn copy_to_systemd() {
    println!("enter sudo pw");
    std::io::stdout().flush().unwrap();
    // NOTE: copy from tmp to systemd dir
    let _child = Command::new("sudo")
        .arg("cp")
        .arg("/tmp/media-nasremote-music.mount")
        .arg("/etc/systemd/system")
        .stdin(Stdio::inherit())
        .output() // NOTE: importand for catching stdin
        .expect("failed to run copy command");
}
