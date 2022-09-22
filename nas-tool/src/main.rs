use std::io::{self, stdin};

use clap::Parser;

/// viewable inside a web browser GUI
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

fn main() -> Result<(), &'static str> {
    let default_ip = String::from("192.168.1.14");

    println!("ff or chrome ?\n1: ff\n2: chrome");
    // TODO: back to index
    println!("h: back to home");
    println!("Enter: select");
    // NOTE: synology api ?
    //
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    print!("{}", line);

    Ok(())
}
