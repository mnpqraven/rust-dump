pub mod builder;
pub mod ip_process;
pub mod tmp_worker;
use std::fs::{self, File};
use std::io::{self, Write};
use std::net::Ipv4Addr;
use std::path::Path;
use std::process::Command;

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

static TMP_USERS: &'static str = "/tmp/users";

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
    /// It will read a cache file containing a list of users in /tmp/users if
    /// such file exists, otherwise it will ssh into the nas and generate one
    fn grab_users(&self) -> Vec<User> {
        match Path::new(&TMP_USERS).try_exists() {
            // read cache for list
            Ok(true) => {
                println!("found cache file, reading...");
                let mut users = Vec::new();
                let file = fs::read_to_string(&TMP_USERS).expect("can't read file, check perms");
                for line in file.lines() {
                    println!("found user {}", line);
                    users.push(User::new(line));
                }
                users
            }
            // grab from ssh
            Err(_) | Ok(false) => {
                // hardcode for now
                // TODO: refactor hardcode
                println!("creating cache file...");
                let ssh: String = format!("{}@{}", "othi".to_string(), self.ip.to_string());
                let me = Command::new("ssh")
                    .arg("-p")
                    .arg(self.port.to_string())
                    .arg(ssh)
                    .arg("ls")
                    .arg("..")
                    // HACK: no more stdin pipe, ssh gets user input from
                    // tty, too much of a PITA to code sshpass
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
                self.create_tmp_users()
                    .expect("can't create tmp user file, check perms");
                users
            }
        }
    }

    fn create_tmp_users(&self) -> Result<(), io::Error> {
        let mut file = File::create(&TMP_USERS).expect("can't create file, check perms");
        for user in &self.users {
            println!("{}", user.name);
            file.write(user.name.as_bytes()).unwrap();
            file.write(b"\n").unwrap();
        }
        Ok(())
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
        let f1 = Nas::connect("127.0.0.1", 6661);
        assert_eq!(f1.hostname, None);
        let f2 = Nas::connect("othiremote.synology.me", 6661);
        assert_eq!(f2.hostname, Some("othiremote.synology.me".to_string()));
        assert_eq!(f2.users, vec![User::new("othi")]);
    }
}
