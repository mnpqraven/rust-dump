pub mod builder;
pub mod exclude;
pub mod ip_process;
use std::process::Command;
use std::net::Ipv4Addr;
use rpassword::read_password;

use ip_process::ip_process::find_ip;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    name: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Nas {
    ip: Ipv4Addr,
    port: u16,
    hostname: Option<String>,
    users: Vec<User>,
}

impl Nas {
    pub fn connect(host: &str, port: u16) -> Self {
        if let Ok((ip, hostname)) = find_ip(host) {
            let nas = Nas {
                ip,
                hostname,
                port,
                users: Vec::new(),
            };
            Nas {
                users: nas.grab_users(),
                ..nas
            }
        } else {
            panic!("AddrParseError ree, look at source code")
        }
    }

    // NOTE: port should not be hardcoded later down the line
    /// Get the list of users in the nas drive, excluding admin
    fn grab_users(&self) -> Vec<User> {
        let pw = read_password().unwrap();
        let ssh: String = format!("admin@{}", self.ip.to_string());
        let me = Command::new("ssh")
            .arg("-p")
            .arg(self.port.to_string())
            .arg(ssh)
            .arg("ls")
            .arg("..")
            // TODO: pipe pw to stdin
            .output()
            .expect("failed to run process");
        // std has the dirs
        let out = String::from_utf8(me.stdout).unwrap();
        let mut users = Vec::new();
        for line in out.lines() {
            match line {
                // excludes these 2
                "@eaDir" | "admin" => {}
                _ => users.push(User::new(line)),
            }
        }
        users
    }
}

impl User {
    fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rpassword::read_password;

    use crate::ip_process::ip_process::find_ip;
    use crate::{Nas, User};

    #[test]
    fn user_vec() {
        let hostname = "othiremote.synology.me";
        let ip = find_ip(hostname).unwrap();
        let nas = Nas {
            ip: ip.0,
            hostname: ip.1,
            port: 6661,
            users: Vec::new(),
        };
        let vec1 = nas.grab_users();
        let vec2 = vec![User::new("othi")];
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn connect_verbose_ip() {
        // let pw = read_password().unwrap();
        let f1 = Nas::connect("127.0.0.1", 6661);
        assert_eq!(f1.hostname, None);
        let f2 = Nas::connect("othiremote.synology.me", 6661);
        assert_eq!(f2.hostname, Some("othiremote.synology.me".to_string()));
        assert_eq!(
            f2.users,
            vec![User::new("othi")]
        );
    }
}
