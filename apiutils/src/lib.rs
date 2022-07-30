use json;
use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tokio::fs as tokiofs;

static TOKEN: &'static str = "token";

// NOTE: De-/Serialization between json and rust struct
// INFO: rename to change naming convention
// #[serde(rename = "userId")]
// user_id: i32
#[derive(Debug, Serialize, Deserialize)]
struct TokenChunk {
    code: i8,
    result: Token,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarkerChunk {
    code: i8,
    result: Vec<Marker>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Marker {
    id: String,
    code: String,
    name: String,
    marker_id: String,
    // __text: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAuth {
    account: String,
    password: String,
}

pub async fn auth() -> Result<String, reqwest::Error> {
    let host = fs::read_to_string("private-dump/login-auth").expect(
        "can't find host file containing the auth url\nmake sure the file exists in ./private-dump",
    );
    let client = reqwest::Client::new();

    let cred_file = fs::read_to_string("private-dump/user_auth.json")
        .expect("can't read user_auth.json\nmake sure the file exists in ./private-dump");
    let cred_obj = json::parse(&cred_file).unwrap();

    let user = UserAuth {
        account: cred_obj["account"].to_string(),
        password: cred_obj["password"].to_string(),
    };

    let response: TokenChunk = client
        .post(host)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&user)
        .send()
        .await?
        .json()
        .await
        .expect("not a valid request, double check your url and your json payload");

    // TODO: test
    tokiofs::write("private-dump/token", response.result.token.clone())
        .await
        .expect("error writing to file");

    Ok(response.result.token)
}

// TODO: work
pub async fn markerss() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    // TODO: make fs token handler
    let host = std::fs::read_to_string("private-dump/markerss-host")
        .expect("can't find pointss-host file containing the auth url\nmake sure the file exists in ./private-dump");
    let token_val: String;
    if Path::new("private-dump/token").exists() {
        println!("using token file ...");
        token_val = std::fs::read_to_string("private-dump/token").expect("can't open file");
    } else {
        println!("not token file found, submitting user authentication");
        token_val = auth().await.expect("authentication failed");
    }

    let response = client
        .get(host)
        .header(TOKEN, token_val)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
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

    let data: MarkerChunk = response.unwrap().json().await?;
    for item in data.result {
        println!(
            "id: {:?} name: {:?} code: {:?}",
            item.id, item.name, item.code
        );
    }
    // println!("{:?}", response);
    Ok(())
}

#[cfg(test)]
mod tests {}
