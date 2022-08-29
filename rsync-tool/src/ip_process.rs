use dns_lookup::lookup_host;
use std::net::{AddrParseError, IpAddr, Ipv4Addr};

/// get ip from domain name
/// return a tuple of ip and provided domain name, the domain is None if ip is provided
///
/// * `host`: either the domain name or the ip address
pub fn find_ip(host: &str) -> Result<(Ipv4Addr, Option<String>), AddrParseError> {
    match host.parse::<IpAddr>() {
        // is already an ip
        Ok(_) => Ok((
            host.parse::<Ipv4Addr>()
                .expect("can't parse given ip to ipv4"),
            None,
        )),
        // is a domain
        _ => {
            let ips = lookup_host(host).unwrap();
            let v4: Ipv4Addr = ips
                .iter()
                .find(|ip| ip.is_ipv4())
                .expect("no ip4 found")
                .to_string()
                .parse::<Ipv4Addr>()
                .expect("can't parse the found ip into ipv4");
            Ok((v4, Some(host.to_owned())))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::find_ip;

    #[test]
    fn lookup_localhost() {
        let ip: Ipv4Addr = "127.0.0.1".parse().unwrap();
        assert_eq!(find_ip("localhost").unwrap().0, ip);
    }
}
