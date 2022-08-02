// TODO: join to one table
pub fn get_host(is_remote: bool) -> Result<String, &'static str> {
    let find_err = String::from("can't find local-host file containing the auth url\nmake sure the file exists in ./private-dump");
    let host_local = std::fs::read_to_string("private-dump/local-host").expect(&find_err);
    let host_remote = std::fs::read_to_string("private-dump/remote-host").expect(&find_err);
    if is_remote {
        Ok(host_remote)
    } else {
        Ok(host_local)
    }
}
