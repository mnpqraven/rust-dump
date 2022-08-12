use std::fs;
use std::process::{Command, Stdio};
use std::{fs::File, io::Write};
use std::net::{self, IpAddr};
use dns_lookup::lookup_host;

use regex::Regex;

// TODO: LISTS
// TODO: should struct NAS has a mountpoint field ?
// TODO: existence of mountpoint should be checked
// TODO: hostname should also be checked in the mountpoint
pub struct NAS {
    _host: String,
    pub ip: net::IpAddr,
    _mountpoint: File,
}
impl NAS {
    pub fn new(_host: String, ip: net::IpAddr, _mountpoint: File) -> Self {
        Self {
            _host,
            ip,
            _mountpoint,
        }
    }
}

static IP_RE: &'static str = r"^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])$";
static WHAT_RE: &'static str =r"(What\s*=\s*//)(((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9]))";
static NAME_RE: &'static str = r"^(.*)/([a-zA-Z-]*.mount)$";

// INFO: should be passed
pub fn check_ip(nas: &NAS) -> Result<bool, &str> {
    let re = Regex::new(IP_RE) .unwrap();
    match re.is_match(&nas.ip.to_string()) {
        true => Ok(true),
        _ => Err("invalid ip addr"),
    }
}
/// replaces the mountpoint file, changing the ip address in the what field
/// Returns String of modified mountpoint file
/// * `mountpoint`: path of the mountpoint file
/// * `new_ip`: ip to replace
pub fn what_replace(mountpoint: String, new_ip: IpAddr) -> String {
    let content = String::from(fs::read_to_string(mountpoint).unwrap());
    let mut result = String::new();

    let what_re = Regex::new(WHAT_RE).unwrap();
    for line in content.lines() {
        if what_re.is_match(line) {
            let caps = what_re.captures(line).unwrap();
            result.push_str(&line.clone().replace(caps.get(2).unwrap().as_str(), &new_ip.to_string()));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub fn file_inject(replace_with: String) -> Result<(), &'static str> {
    let mut file =
        File::create("/tmp/media-nasremote-music.mount").expect("can't create a new file");
    file.write_all(replace_with.as_bytes())
        .expect("can't write to file");
    println!("copying with sudo cp /tmp/media-nasremote-music.mount /etc/systemd/system/media-nasremote-music.mount");
    Ok(())
}

#[allow(dead_code)]
struct MOUNT {
    path: String,
    filename: String,
}
#[allow(dead_code)]
impl MOUNT {
    pub fn new(fullpath: String) -> Self {
        // INFO: fullpath example: "/etc/systemd/system/media-nasremote-music.mount"
        // extra `-` charcter for systemd's dir traversing naming scheme
        // cap group 0: fullpath
        // cap group 1: dir (/etc/systemd/system)
        // cap group 2: filename (media-nasremote-music.mount)
        let name_regex = Regex::new(NAME_RE).unwrap();
        let captures = name_regex.captures(&fullpath).unwrap();
        let path = captures.get(1).map_or("", |m| m.as_str());
        let filename = captures.get(2).map_or("", |m| m.as_str());
        Self {
            path: path.to_owned(),
            filename: filename.to_owned(),
        }
    }
}
/// Copy the mountpoint file generated in /tmp to systemd dir
pub fn copy_to_systemd() {
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
// TODO: implement multiple args/file's existence, for now just media-nasremote-music.mount
fn _lookup_filename(path: String) -> Result<(), &'static str> {
    match fs::read_dir(path) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

/// get the domain's public IPv4 address
///
/// * `domain`: domain name to grab IP address
pub fn lookup_v4(domain: String) -> net::IpAddr {
    let ips: Vec<std::net::IpAddr> = lookup_host(&domain).unwrap();
    *ips.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ip_pass() {
        let good_ips = ["182.221.23.1", "192.168.1.1"];
        let path = "/etc/systemd/system/media-nasremote-music.mount";
        for ip in good_ips {
            let nas = NAS::new(
                "random host".to_string(),
                ip.parse().unwrap(),
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
                ip.parse().unwrap(),
                File::open(path).expect("can't open file"),
            );
            assert_eq!(check_ip(&nas).unwrap(), false);
        }
    }

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
}
