use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE},
};
use std::{fs, path::Path};

mod obj;
mod util;
use obj::group::*;
use obj::marker::*;
use obj::system::*;
use util::auth::*;
use util::file::*;

static TOKEN: &'static str = "token";

// INFO: works
pub async fn markerss(limit: u16) -> Result<Vec<Marker>, reqwest::Error> {
    let client = reqwest::Client::new();

    // TODO: make fs token handler
    // TODO: refactor
    let host = std::fs::read_to_string("private-dump/markerss-host")
        .expect("can't find pointss-host file containing the auth url\nmake sure the file exists in ./private-dump");
    let token_val: String;
    if Path::new("private-dump/token").exists() {
        println!("using token file ...");
        token_val = std::fs::read_to_string("private-dump/token").expect("can't open file");
    } else {
        token_val = auth().await?;
    }

    let response = client
        .get(host)
        .header(TOKEN, token_val)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .query(&[("$limit", limit)])
        .send()
        // TODO: handle invalid token here
        .await;
    if response.is_err() {
        println!("invalid token file, reauthing");
        // for now
        auth().await.expect("cannot reauth");
        println!("reauth attempted, rerun the program\nif it still doesn't work, delete the token");
        // TODO: resend request
    }

    let data: MarkerCollection = response.unwrap().json().await?;
    // INFO: debug
    for item in &data.result {
        println!(
            "{:?} | {:?} | {:?}",
            item.id, item.name, item.code, /* item.__text.as_ref().unwrap() */
        );
    }
    // println!( "------------ID-------------|------NAME------|---CODE---|");
    Ok(data.result)
}

/// currently this function will add all avaible item in the vector struct
pub async fn permission_patch(id: String) -> Result<(), reqwest::Error> {
    let data = markerss(10000)
        .await
        .expect("can't get the list of markers");

    let mut marker_ids = Vec::new();
    for item in data {
        marker_ids.push(item.id);
    }

    let payload = Permission {
        // TODO: refactor hardcode
        id: id.to_owned(),
        // FIX: leaving this empty deletes the registry
        // can grab these directly from GET permissions
        group_id: None,
        user_id: None,
        marker_ids,
    };

    let client = reqwest::Client::new();

    let mut permission_host: String = get_host(true).unwrap();
    permission_host.push_str("permissions/");
    permission_host.push_str(&id);

    let response: PatchResponse = client
        .patch(permission_host)
        // TODO: make auth handle error and not panic
        .header(TOKEN, auth().await?)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&payload)
        .send()
        .await?
        .json()
        .await
        .expect("not a valid request, double check your url and your json payload");

    println!("{:?}", response);
    Ok(())
}

/// this returns created_at, id, member_ids, name, updated_at, __text
/// TODO: should join with member_ids to get members
/// TODO: should join with permissions to get group perms
pub async fn groups(limit: u16) -> Result<Vec<Group>, reqwest::Error> {
    let client = reqwest::Client::new();

    // TODO: make fs token handler
    // TODO: refactor
    let mut group_host: String = get_host(true).unwrap();
    group_host.push_str("groups/");

    let response = client
        .get(group_host)
        .header(TOKEN, auth().await?)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .query(&[("$limit", limit)])
        .send()
        // TODO: handle invalid token here
        .await;
    if response.is_err() {
        println!("invalid token file, reauthing");
        // for now
        auth().await.expect("cannot reauth");
        println!("reauth attempted, rerun the program\nif it still doesn't work, delete the token");
        // TODO: resend request
    }

    let data: GroupCollection = response.unwrap().json().await?;
    // INFO: debug
    for item in &data.result {
        println!("{:?} |{:?}", item.id, item.name);
    }
    // println!( "------ ID------|-------NAME------");
    Ok(data.result)
}
pub fn wipe_token() -> Result<(), std::io::Error> {
    fs::remove_file("private-dump/token")
}
pub fn write_csv() -> Result<(), &'static str> {
    Ok(())
}

#[cfg(test)]
mod tests {}
