use clap::{Arg, App};
use regex::Regex;
use std::process::Command;

struct NAS {
    host: String,
    ip: String
}
impl NAS {
    fn new(host: String, ip: String) -> Self {
        Self {
            host,
            ip
        }
    }
}
// TODO: implement autolookup, for now just pass the new IP as args
// TODO: implement multiple args, for now just media-nasremote-music.mount
fn main() {
    let host = "othiremote.synology.me";
    // hardcode for now
    let mountpoint = "/etc/systemd/system/media-nasremote-music.mount";

    let app = App::new("naswiz")
        .version("0.1.0")
        .author("othi")
        .about("nas mountpoint update wizard")
        .arg(
            Arg::new("ip")
            .help("nas' public ip address")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            ).get_matches()
        ;


    if let Some(ip) = app.value_of("ip") {
        let nas = NAS::new(host.to_string(), ip.to_string());
        if check_ip(&nas).unwrap() {
            println!("{} is a valid ip address (a valid ip addr starts with 123)", nas.ip);
            println!("initialized: host {} with ip {}", nas.host, nas.ip);
        } else {
            panic!("not a valid ip");
        }
    }

    let output = Command::new("cat")
        .arg(mountpoint)
        .output()
        .expect("couldn't execute the process");
    let output_readable = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", output_readable);
}

// INFO: passed
fn check_ip(nas: &NAS) -> Result<bool, &str> {
    let re = Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}").unwrap();
    match re.is_match(&nas.ip) {
        true => Ok(true),
        _ => Err("invalid ip addr")
    }
}

// TODO: only the hostname after Where until EOL
fn _get_where(cat: String) -> String{
    let _what_i_want = cat;
    todo!();
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::{NAS, check_ip};

    #[test]
    fn check_ip_pass() {
        let good_ip1 = "182.221.23.1";
        let good_ip2 = "192.168.1.1";
        let nas1 = NAS::new("random host".to_string(), good_ip1.to_string());
        let nas2 = NAS::new("random host".to_string(), good_ip2.to_string());
        assert_eq!(check_ip(&nas1).unwrap(), true);
        assert_eq!(check_ip(&nas2).unwrap(), true);
    }

    #[test]
    #[should_panic(expected = "invalid ip addr")]
    fn check_ip_fail() {
        let bad_ip1 = "278.12.23.2";
        let bad_ip2 = "123.-1.23.2";
        let bad_ip3 = "278.-1.23.2";
        let bad_ip4 = "21.23.231";
        let nas1 = NAS::new("random host".to_string(),bad_ip1.to_string());
        let nas2 = NAS::new("random host".to_string(),bad_ip2.to_string());
        let nas3 = NAS::new("random host".to_string(),bad_ip3.to_string());
        let nas4 = NAS::new("random host".to_string(),bad_ip4.to_string());
        assert_eq!(check_ip(&nas1).unwrap(), true);
        assert_eq!(check_ip(&nas2).unwrap(), true);
        assert_eq!(check_ip(&nas3).unwrap(), true);
        assert_eq!(check_ip(&nas4).unwrap(), true);
    }

    #[test]
    fn ip_regex() {
        let bad_ip1 = "278.12.23.2";
        let bad_ip2 = "123.-1.23.2";
        let bad_ip3 = "278.-1.23.2";
        let bad_ip4 = "21.23.231";
        let good_ip1 = "182.221.23.1";
        let good_ip2 = "192.168.1.1";

        let re = Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}").unwrap();
        assert!(re.is_match(good_ip1));
        assert!(re.is_match(good_ip2));
        // TODO: implement false
        assert_eq!(re.is_match(bad_ip1), false);
        assert_eq!(re.is_match(bad_ip2), false);
        assert_eq!(re.is_match(bad_ip3), false);
        assert_eq!(re.is_match(bad_ip4), false);
    }
}
