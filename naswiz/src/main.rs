use clap::Parser;
use dns_lookup::LookupError;
use naswiz::check_ip;
use naswiz::gen_new_file;
use naswiz::lookup;
use naswiz::NAS;
use dns_lookup::lookup_host;
use std::fs::File;
use std::net::AddrParseError;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// nas' public ip address or domain
    #[clap(value_parser)]
    ip: String,

    /// the mount file, e.g if your mountpoint is in /etc/systemd/system/media-nasremote-music.mount
    /// then the argument should be media-nasremote-music
    #[clap(short, long, value_parser)]
    mount: String,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    let mountpoint = format!("/etc/systemd/system/{}.mount", args.mount);
    println!("{}", mountpoint);

    // NOTE: RUNTIME
    // INFO: currently cr -m mount ip or cr -m mount --host host
    // TODO: refactor error handling, unify 2 arguments
    let rando = String::from("othiremote.synology.me");
    let mut ip: Result<std::net::IpAddr, AddrParseError> = rando.parse();
    match ip {
        Ok(_) => ip = args.ip.parse().expect("not a valid ip address"),
        Err(x) => {
                ip = lookup(
                    x
                        .as_ref()
                        .expect("can't parse the host name")
                        .to_string(),
                )
        },
    }

    let nas = NAS::new(
        ip,
        File::open(&mountpoint).expect(
            "can't find said file\n
            Does the file exist ?/Do you have permission to view the file ?",
        ),
    );
    match check_ip(&nas) {
        Ok(_) => {
            println!("{} is a valid ip address, continuing ...", &nas.ip);
            gen_new_file(args.mount.to_string(), nas.ip).unwrap();
            // should eventually pass paths and file name as args
        }
        Err(_) => panic!("not a valid ip address"),
    }
    Ok(())
}
