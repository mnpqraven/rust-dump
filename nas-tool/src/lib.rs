pub mod worker;

use std::{fmt::Display, io, net::Ipv4Addr};

/// TODO: serde
struct Nas {
    ip: Ipv4Addr,
    domain: Option<Ipv4Addr>,
    users: Vec<User>,
    // INFO: should ports be users' children or nas' children?
    // try nas' children first
    ports: Vec<Port>,
}

pub struct Port {
    port: u16,
}

// TODO: need role??
// probably
struct User {
    name: String,
    role: ROLE,
}
enum ROLE {
    Admin,
    Guest,
}
/// find userlist file in /tmp
fn _find_user_list() -> Vec<User> {
    unimplemented!()
}
/// creates a list of user if can't find it in /tmp
fn _create_user_list() -> Result<(), io::Error> {
    unimplemented!();
    Ok(())
}
