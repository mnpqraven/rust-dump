use crate::user_input::build_custom_fmt;
use crate::Nas;
use crate::User;
use std::convert::TryFrom;
use strum_macros::EnumIter;

pub enum HomeType {
    Volume1,
    VarServices,
}
// /var/services/homes/othi = $HOME = /volume1/homes/othi
// NOTE: volume1 changable in future ?
#[derive(Debug, EnumIter)]
pub enum Dir {
    // @/db1
    Db1 = 1,
    // @/NetBackup/othi
    NetBackup = 2,
    // @/homes/othi/music/voice
    Voice = 3,
    // @/music
    Music = 4,
    Custom = 5,
}

impl TryFrom<u8> for Dir {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Dir::Db1),
            2 => Ok(Dir::NetBackup),
            3 => Ok(Dir::Voice),
            4 => Ok(Dir::Music),
            5 => Ok(Dir::Custom),
            _ => Err(()),
        }
    }
}

fn build_ssh(user: User, nas: Nas) -> String {
    format!("{}@{}", user.name, nas.ip)
}

/// builds path from args
/// e.g /volume1/NetBackup/othi
/// or /var/services/NetBackup/othi
fn build_path(dir: Dir, home_type: HomeType, user: User) -> String {
    let voicefmt = format!("homes/{}/music/voice", user.name);
    let customfmt: String = build_custom_fmt();
    let folder: &str = match dir {
        Dir::Db1 => "db1",
        Dir::NetBackup => "NetBackup",
        Dir::Voice => &voicefmt,
        Dir::Music => "music",
        Dir::Custom => &customfmt,
    };
    let home = match home_type {
        HomeType::Volume1 => "/volume1",
        HomeType::VarServices => "/var/services",
    };
    match dir {
        Dir::Db1 => format!("/volume1/{}", folder),
        Dir::Custom => format!("{}", folder),
        _ => format!("{}/{}/{}", home, folder, user.name),
    }
}

/// generates remote path including ssh address and file path
pub fn build_target_arg(
    user_ssh: User,
    nas: Nas,
    dir: Dir,
    home_type: HomeType,
    user_client: User,
) -> String {
    format!(
        "{}:{}",
        build_ssh(user_ssh, nas),
        build_path(dir, home_type, user_client)
    )
}
