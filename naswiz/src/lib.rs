use std::fs;
use std::{fs::File, io::Write};

use regex::Regex;

// TODO: LISTS
// TODO: should struct NAS has a mountpoint field ?
// TODO: existence of mountpoint should be checked
// TODO: hostname should also be checked in the mountpoint
pub struct NAS {
    _host: String,
    pub ip: String,
    _mountpoint: File,
}
impl NAS {
    pub fn new(_host: String, ip: String, _mountpoint: File) -> Self {
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
    let re = Regex::new(IP_RE)
        .unwrap();
    match re.is_match(&nas.ip) {
        true => Ok(true),
        _ => Err("invalid ip addr"),
    }
}
pub fn what_replace(mountpoint: String, new_ip: String) -> String {
    let content = String::from(fs::read_to_string(mountpoint).unwrap());
    let mut result = String::new();

    let what_re = Regex::new(WHAT_RE).unwrap();
    for line in content.lines() {
        if what_re.is_match(line) {
            let caps = what_re.captures(line).unwrap();
            result.push_str(&line.clone().replace(caps.get(2).unwrap().as_str(), &new_ip));
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
// TODO: implement multiple args/file's existence, for now just media-nasremote-music.mount
fn _lookup_filename(path: String) -> Result<(), &'static str> {
    match fs::read_dir(path) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
#[test]
fn regex() {
    let mountpoint = "/etc/systemd/system/media-nasremote-music.mount";
    let name_regex = Regex::new(r"^(.*)/([a-zA-Z-]*.mount)$").unwrap();
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

// TODO: implement host's ip autolookup, for now just pass the new IP as args
fn _lookup(_ip: String) -> String {
    unimplemented!();
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
    }
}
