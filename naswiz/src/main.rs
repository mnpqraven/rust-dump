use clap::{App, Arg};
use naswiz::check_ip;
use naswiz::lookup;
use naswiz::gen_new_file;
use naswiz::NAS;
use std::fs::File;

fn main() -> Result<(), &'static str> {
    // hardcode for now
    let host = "othiremote.synology.me";
    let mountpoint = "/etc/systemd/system/media-nasremote-music.mount";

    // NOTE: meta
    let app = App::new("naswiz")
        .about("Synology NAS mountpoint update wizard\nThis program fetches your public url domain and updates the mountpoint in systemd")
        .arg(
            Arg::new("ip")
                .help("nas' public ip address. Uses ip fetched from synology domain by default")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()), // .required(true),
        )
        .get_matches();

    // NOTE: RUNTIME
    let ip: std::net::IpAddr;
    if let Some(x) =  app.value_of("ip") {
        ip = x.parse().expect("not a valid ip address")
    }
    else {
        ip = lookup(host.to_string())
    }
    let nas = NAS::new(
        host.to_string(),
        ip,
        File::open(mountpoint).expect("can't find said file\nDoes the file exist ?/Do you have permission to view the file ?"));
    match check_ip(&nas) {
        Ok(_) => {
            println!("{} is a valid ip address, continuing ...", &nas.ip);
            gen_new_file(mountpoint.to_string(), nas.ip).unwrap();
            // should eventually pass paths and file name as args
        }
        Err(_) => panic!("not a valid ip address"),
    }
    Ok(())
}
