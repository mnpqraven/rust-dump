use dns_lookup::lookup_host;
use std::fs;
use std::net::{self, IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::{Command, Stdio};
use std::{fs::File, io::Write};

use regex::Regex;

pub struct NAS {
    pub ip: net::Ipv4Addr,
    _mountpoint: File,
}
impl NAS {
    pub fn new(ip: net::Ipv4Addr, _mountpoint: File) -> Self {
        Self { ip, _mountpoint }
    }
}

static WHAT_RE: &'static str = r"(What\s*=\s*//)(((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9]))";
#[allow(dead_code)]
static NAME_RE: &'static str = r"^(.*)/([a-zA-Z-]*.mount)$";

/// replaces the mountpoint file, changing the ip address in the what field
///
/// * `mountpoint`: path of the mountpoint file
/// * `new_ip`: ip to replace
pub fn gen_new_file(mount: String, new_ip: Ipv4Addr) -> Result<(), &'static str> {
    let mountpoint = format!("/etc/systemd/system/{}.mount", mount);
    let tmp_mountpoint = format!("/tmp/{}.mount", mount);
    let content = String::from(fs::read_to_string(&mountpoint).unwrap());
    let mut result = String::new();

    let what_re = Regex::new(WHAT_RE).unwrap();
    for line in content.lines() {
        if what_re.is_match(line) {
            let caps = what_re.captures(line).unwrap();
            result.push_str(
                &line
                    .clone()
                    .replace(caps.get(2).unwrap().as_str(), &new_ip.to_string()),
            );
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    let mut file = File::create(&tmp_mountpoint).expect("can't create a new file");
    file.write_all(result.as_bytes())
        .expect("can't write to file");
    println!("copying with sudo cp {} {}", &tmp_mountpoint, &mountpoint);

    copy_to_systemd(mount);
    Ok(())
}

/// Copy the mountpoint file generated in /tmp to systemd dir
fn copy_to_systemd(mount: String) {
    std::io::stdout().flush().unwrap();
    // NOTE: copy from tmp to systemd dir
    let child = Command::new("sudo")
        .arg("cp")
        .arg(format!("/tmp/{}.mount", mount))
        .arg("/etc/systemd/system")
        .stdin(Stdio::inherit())
        .output() // NOTE: importand for catching stdin
        .expect("failed to run copy command");
    match child.status.success() {
        true => println!("copy success"),
        _ => panic!("copy failed")
    }
}

/// get the domain's public IPv4 address
///
/// * `domain`: domain name to grab IP address
pub fn lookup(domain: String) -> Result<Ipv4Addr, std::net::AddrParseError> {
// Result<net::Ipv4Addr, std::io::Error> {
    let ips: Result<Vec<net::IpAddr>,std::io::Error> = lookup_host(&domain);
    match ips {
        Ok(_) => {
            let mut ip4s: Vec<Ipv4Addr> = Vec::new();
            let mut ip6s: Vec<Ipv6Addr> = Vec::new(); // unused for now
            for ip in ips.unwrap() {
                match ip {
                    IpAddr::V4(ipv4) => ip4s.push(ipv4),
                    IpAddr::V6(ipv6) => ip6s.push(ipv6)
                }
            }
            Ok(ip4s.first().cloned().unwrap())
        },
        Err(error) =>  panic!("{}", error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex() {
        let mountpoint = "/etc/systemd/system/media-nasremote-music.mount";
        let name_regex = Regex::new(NAME_RE).unwrap();
        let captures = name_regex.captures(mountpoint).unwrap();
        assert_eq!(
            captures.get(1).map_or("", |m| m.as_str()),
            "/etc/systemd/system"
        );
        assert_eq!(
            captures.get(2).map_or("", |m| m.as_str()),
            "media-nasremote-music.mount"
        );
    }

    #[test]
    fn look() {
        let hostname = String::from("othiremote.synology.me");
        let ips: Vec<net::IpAddr> = lookup_host(&hostname).unwrap();
        assert!(ips.contains(&"118.71.111.229".parse().unwrap()));
        // ips[0] and [1] for IpV4 and IpV6
        println!("{:?}", ips[0]);
    }

    #[test]
    fn fmt() {
        let mount = String::from("media-nasremote-music");
        let f = format!("/etc/systemd/system/{}.mount", mount);
        assert_eq!(
            f,
            "/etc/systemd/system/media-nasremote-music.mount".to_string()
        );
    }
}
