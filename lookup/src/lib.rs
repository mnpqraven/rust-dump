use std::net::Ipv4Addr;
use dns_lookup::lookup_host;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn lookupv4(host: &str) -> Vec<Ipv4Addr> {
    let mut ips: Vec<Ipv4Addr> = Vec::new();
    for item in &lookup_host(host).unwrap() {
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
        let ips: Vec<Ipv4Addr> = vec!["127.0.0.1".parse().unwrap()];
        assert_eq!(lookupv4("localhost"),ips);
    }
}
