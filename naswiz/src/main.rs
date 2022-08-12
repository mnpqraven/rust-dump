use clap::{App, Arg};
use naswiz::check_ip;
use naswiz::lookup_v4;
use std::fs::File;

use naswiz::copy_to_systemd;
use naswiz::file_inject;
use naswiz::what_replace;
use naswiz::NAS;

fn main() -> Result<(), &'static str> {
    // hardcode for now
    let host = "othiremote.synology.me";
    let mountpoint = "/etc/systemd/system/media-nasremote-music.mount";

    // NOTE: meta
    let app = App::new("naswiz")
        .about("Synology NAS mountpoint update wizard\nThis program fetches your public url domain and updates the mountpoint in systemd")
        .arg(
            Arg::new("ip")
                .help("nas' public ip address")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()), // .required(true),
        )
        .get_matches();

    // NOTE: RUNTIME
    let nas: NAS;
    match app.value_of("ip") {
        Some(ip) => {
            nas = NAS::new(
                host.to_string(),
                ip.parse().unwrap(),
                File::open(mountpoint).expect("can't find said file\nDoes the file exist ?/Do you have permission to view the file ?"));
        },
        None => {
            nas = NAS::new(
                host.to_string(),
                lookup_v4(host.to_string()),
                File::open(mountpoint).expect("can't find said file\nDoes the file exist ?/Do you have permission to view the file ?"));
        }
    }
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
    Ok(())
}
