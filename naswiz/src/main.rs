use clap::{App, Arg};
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    process::{Command, Stdio},
};

// TODO: should this has a mountpoint field ?
struct NAS {
    _host: String,
    ip: String,
    _mountpoint: File,
}
impl NAS {
    fn new(_host: String, ip: String, _mountpoint: File) -> Self {
        Self {
            _host,
            ip,
            _mountpoint,
        }
    }
}
fn main() {
    let host = "othiremote.synology.me";
    // hardcode for now
    let mountpoint = "/etc/systemd/system/media-nasremote-music.mount";

    // NOTE: meta
    let app = App::new("naswiz")
        .version("0.1.0")
        .author("othi")
        .about("nas mountpoint update wizard")
        .arg(
            Arg::new("ip")
                .help("nas' public ip address")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .get_matches();

    // NOTE: RUNTIME
    if let Some(ip) = app.value_of("ip") {
        let nas = NAS::new(
            host.to_string(),
            ip.to_string(),
            File::open(mountpoint).expect("can't find said file"),
        );
        match check_ip(&nas) {
            Ok(_) => {
                println!("{} is a valid ip address, continuing ...", &nas.ip);
                let dump = what_replace(mountpoint.to_string(), nas.ip).to_string();
                file_inject(dump).expect("can't write to file");

                println!("enter sudo pw");
                std::io::stdout().flush().unwrap();
                // NOTE: copy from tmp to systemd dir
                let _child = Command::new("sudo")
                    .arg("cp")
                    .arg("/tmp/media-nasremote-music.mount")
                    .arg("/etc/systemd/system")
                    .stdin(Stdio::inherit())
                    .output() // NOTE: importand for catching stdin
                    .expect("failed reverse command");
            }
            Err(_) => panic!("not a valid ip address"),
        }
    }
}

// INFO: should be passed
fn check_ip(nas: &NAS) -> Result<bool, &str> {
    let re = Regex::new(r"^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])$")
        .unwrap();
    match re.is_match(&nas.ip) {
        true => Ok(true),
        _ => Err("invalid ip addr"),
    }
}

fn what_replace(mountpoint: String, new_ip: String) -> String {
    let content = String::from(fs::read_to_string(mountpoint).unwrap());
    // let mut left_find = String::new();
    let mut result = String::new();

    let fs_regex = Regex::new(r"(What\s*=\s*//)(((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9]))").unwrap();
    for line in content.lines() {
        if fs_regex.is_match(line) {
            // NOTE: debug
            // println!("found What line {}", line);

            // left_find = line.clone().to_string();
            let caps = fs_regex.captures(line).unwrap();
            result.push_str(&line.clone().replace(caps.get(2).unwrap().as_str(), &new_ip));
            // println!("new replaced output: {}", result);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

fn file_inject(replace_with: String) -> Result<(), &'static str> {
    let mut file =
        File::create("/tmp/media-nasremote-music.mount").expect("can't create a new file");
    file.write_all(replace_with.as_bytes())
        .expect("can't write to file");
    println!("file created, for now do\nsudo cp /tmp/media-nasremote-music.mount /etc/systemd/system/media-nasremote-music.mount");
    Ok(())
}

// TODO: implement host's ip autolookup, for now just pass the new IP as args
fn _lookup(_ip: String) -> String {
    unimplemented!();
}
// TODO: implement multiple args, for now just media-nasremote-music.mount
fn _lookup_filename(_filename: String) -> String {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::fs::{self, File};

    use crate::{check_ip, NAS};

    #[test]
    fn check_ip_pass() {
        let good_ips = ["182.221.23.1", "192.168.1.1"];
        let path = "/etc/systemd/system/media-nasremote-music.mount";
        for ip in good_ips {
            let nas = NAS::new(
                "random host".to_string(),
                ip.to_string(),
                File::open(path).expect("can't open file"),
            );
            assert_eq!(check_ip(&nas).unwrap(), true);
        }
    }
    #[test]
    #[should_panic(expected = "invalid ip addr")]
    fn check_ip_fail() {
        let bad_ips = [
            "278.12.23.2",
            "123.-1.23.2",
            "278.-1.23.2",
            "21.23.231",
            "121.222.1.2",
        ];
        let path = "/etc/systemd/system/media-nasremote-music.mount";
        for ip in bad_ips {
            let nas = NAS::new(
                "random host".to_string(),
                ip.to_string(),
                File::open(path).expect("can't open file"),
            );
            assert_eq!(check_ip(&nas).unwrap(), false);
        }
    }
    // TODO: refactor to unit test
    #[test]
    fn file_ip_replace() {
        let mountpoint = "/etc/systemd/system/media-nasremote-music.mount";
        let content = String::from(fs::read_to_string(mountpoint).unwrap());
        let mut left_find = String::new();

        // NOTE: capture group 2 is the ip, cap gr 1 is What=//
        let fs_regex = Regex::new(r"(What\s*=\s*//)((\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3})").unwrap();
        for line in content.lines() {
            if fs_regex.is_match(line) {
                println!("found What line {}", line);

                left_find = line.clone().to_string();

                let caps = fs_regex.captures(line).unwrap();
                assert_eq!(caps.get(2).unwrap().as_str(), "42.115.6.173");
                let new_ip = "11.11.11.11";
                let result = line.replace(caps.get(2).unwrap().as_str(), new_ip);
                assert_eq!(result, "What=//11.11.11.11/music");
                println!("new replaced output: {}", result);
                break;
            }
        }

        let right_find = "What=//42.115.6.173/music";
        assert_eq!(left_find, right_find);
        // content.into_iter().find(fs_regex)
    }
}
