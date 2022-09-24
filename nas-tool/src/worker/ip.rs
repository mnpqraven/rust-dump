use dns_lookup::{lookup_host, LookupError};
use std::{
    io,
    net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr}, fmt::Display,
};

use crate::Port;

/// viewable ins&ide a web browser GUI
trait Viewable {
    fn open_link();
}
/// port can be connected via remote IP connection
trait RemoteConnectablePort {}
/// port can be connected via local IP connection
trait LocalConnectablePort {}

// u16
enum GuiPort {
    HttpGui = 5000,
    HttpsGui = 5001,
    CalibreClient = 7080,
    CalibreDb = 7081,
    Utorrent = 8080,
}
impl Viewable for GuiPort {
    fn open_link() {
        println!("opening link..");
    }
}

enum OtherPort {
    SSH = 6661,
}

fn lookup_v4(hostname: &str) -> Result<Ipv4Addr, AddrParseError> {
    match lookup_host(hostname) {
        Ok(ips) => {
            let mut vec_v4 = Vec::new();
            for ip in ips {
                if ip.is_ipv4() {
                    vec_v4.push(ip.to_string().parse::<Ipv4Addr>().unwrap());
                }
            }
            Ok(vec_v4.first().unwrap().clone())
        }
        Err(err) => panic!("{}", err),
    }
}
impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.port)
    }
}

pub fn build_full_link( hostname: &str, port: Option<Port>, http_needed: bool,
) -> Result<String, &'static str> {
    let address = lookup_v4(hostname).unwrap();
    let mut http = String::new();
    let mut p = String::new();
    match port {
        Some(x) => {
            p.push(':');
            p.push_str(&x.to_string());
        },
        None => {}
    }
    if http_needed {
        http.push_str("http://");
    }
    // 192.168.1.14:5000
    Ok(format!("{}{}{}", http, address, p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {}

    #[test]
    fn lookup_localhost() {
        let localhost_ip = lookup_v4("localhost");
        assert_eq!(localhost_ip, "127.0.0.1".parse::<Ipv4Addr>())
    }

    #[test]
    fn link_generate() {
        let hostname = "localhost";
        let link = build_full_link(hostname, Some(Port { port: 5000}), true).unwrap();
        assert_eq!(link, "http://127.0.0.1:5000");

        let link = build_full_link(hostname, Some(Port { port: 5000}), false).unwrap();
        assert_eq!(link, "127.0.0.1:5000");
    }
}
