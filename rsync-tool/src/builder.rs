use crate::Nas;
use crate::User;

pub enum HomeType {
    Volume,
    VarServices,
}
// /var/services/homes/othi = $HOME = /volume1/homes/othi
// NOTE: volume1 changable in future ?
pub enum Dir {
    // /volume1/db1
    Db,
    // /volume1/NetBackup/othi
    // /var/services/NetBackup/othi
    NetBackup,
    // /var/services/homes/othi/music/voice
    // /volume1/homes/othi/music/voice
    Voice,
    // /var/services/music
    // /volume1/music
    Music,
}

fn build_ssh(user: User, nas: Nas) -> String {
    format!("{}@{}", user.name, nas.ip)
}

/// builds path from args
/// e.g /volume1/NetBackup/othi
/// or /var/services/NetBackup/othi
fn build_path(folder: &str, dir: Dir, home_type: HomeType, user: User) -> String {
    let home = match home_type {
        HomeType::Volume => "/volume1",
        HomeType::VarServices => "/var/services",
    };
    match dir {
        Dir::Db => format!("{}/{}", home, folder),
        _ => format!("{}/{}/{}", home, folder, user.name),
    }
}

pub fn build_target_arg(
    user_ssh: User,
    nas: Nas,
    folder: &str,
    dir: Dir,
    home_type: HomeType,
    user_client: User,
) -> String {
    format!(
        "{}:{}",
        build_ssh(user_ssh, nas),
        build_path(folder, dir, home_type, user_client)
    )
}
