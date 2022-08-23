use clap::Parser;
use naswiz::gen_new_file;
use naswiz::lookup;
use naswiz::NAS;
use std::fs::File;

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
    let ip = lookup(args.ip);

    let nas = NAS::new(
        ip.unwrap(),
        File::open(&mountpoint).expect(
            "can't find said file\n
            Does the file exist ?/Do you have permission to view the file ?",
        ),
    );
    println!("{} is a valid ip address, continuing ...", &nas.ip);
    gen_new_file(args.mount.to_string(), nas.ip).unwrap();
    Ok(())
}
