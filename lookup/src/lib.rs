use std::net::{Ipv4Addr, IpAddr};
use dns_lookup::lookup_host;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn lookup(host: &str) -> Vec<Ipv4Addr> {
    let res: Vec<IpAddr> = lookup_host(host).unwrap();
    let mut ips: Vec<Ipv4Addr> = Vec::new();
    for item in &res {
        if item.to_string().parse::<Ipv4Addr>().is_ok() {
            ips.push(item.to_string().parse::<Ipv4Addr>().unwrap());
        }
    }
    ips
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ips: Vec<Ipv4Addr> = Vec::new();
        ips.push("127.0.0.1".parse().unwrap());
        assert_eq!(lookup("localhost"),ips);
    }
}
