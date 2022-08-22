use dns_lookup::lookup_host;
use std::fs;
use std::net::{self, IpAddr};
use std::process::{Command, Stdio};
use std::{fs::File, io::Write};

use regex::Regex;

pub struct NAS {
    pub ip: net::IpAddr,
    _mountpoint: File,
}
impl NAS {
    pub fn new(ip: net::IpAddr, _mountpoint: File) -> Self {
        Self { ip, _mountpoint }
    }
}

static IP_RE: &'static str = r"^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])$";
static WHAT_RE: &'static str = r"(What\s*=\s*//)(((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9]))";
static NAME_RE: &'static str = r"^(.*)/([a-zA-Z-]*.mount)$";

// INFO: should be passed
pub fn check_ip(nas: &NAS) -> Result<bool, &str> {
    let re = Regex::new(IP_RE).unwrap();
    match re.is_match(&nas.ip.to_string()) {
        true => Ok(true),
        _ => Err("invalid ip addr"),
    }
}
/// replaces the mountpoint file, changing the ip address in the what field
///
/// * `mountpoint`: path of the mountpoint file
/// * `new_ip`: ip to replace
pub fn gen_new_file(mount: String, new_ip: IpAddr) -> Result<(), &'static str> {
    // mountpoint = format!(/etc/systemd/system/{}.mount, mount)
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

// TODO: implement multiple args/file's existence, for now just media-nasremote-music.mount
fn _lookup_filename(path: String) -> Result<(), &'static str> {
    match fs::read_dir(path) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
/// Copy the mountpoint file generated in /tmp to systemd dir
fn copy_to_systemd(mount: String) {
    std::io::stdout().flush().unwrap();
    // NOTE: copy from tmp to systemd dir
    let _child = Command::new("sudo")
        .arg("cp")
        .arg(format!("/tmp/{}.mount", mount))
        .arg("/etc/systemd/system")
        .stdin(Stdio::inherit())
        .output() // NOTE: importand for catching stdin
        .expect("failed to run copy command");
    match _child.status.success() {
        true => println!("copy success"),
        _ => panic!("copy failed")
    }
}

/// get the domain's public IPv4 address
///
/// * `domain`: domain name to grab IP address
pub fn lookup(domain: String) -> net::IpAddr {
    let ips: Vec<net::IpAddr> = lookup_host(&domain).unwrap();
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
