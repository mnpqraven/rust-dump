use reqwest::{self, header::{CONTENT_TYPE, ACCEPT}};
use serde::{Serialize, Deserialize};
use std::fs;
use json;

static TOKEN: &'static str = "token";

// NOTE: conversion between json and struct
#[derive(Debug, Serialize, Deserialize)]
struct TokenChunk {
    code: i8,
    result: Token
}

#[derive(Debug, Serialize, Deserialize)]
struct MarkerChunk {
    code: i8,
    result: Vec<Marker>
}

#[derive(Debug, Serialize, Deserialize)]
struct Marker {
    id: String,
    lng: Option<f32>,
    lat: Option<f32>,
    code: String,
    name: String,
    marker_id: String,
    videos: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    // hardcoded date for now
    crated_at: Option<String>,
    updated_at: Option<String>,
    __text: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String
}

// INFO: rename to change naming convention
// #[serde(rename = "userId")]
// user_id: i32

#[derive(Debug, Serialize, Deserialize)]
struct UserAuth {
    account: String,
    password: String
}

pub async fn auth() -> Result<String, reqwest::Error>{
    let host = fs::read_to_string("private-dump/login-auth").expect("can't find host file containing the auth url\nmake sure the file exists in ./private-dump");
    let client = reqwest::Client::new();

    let cred_file = fs::read_to_string("private-dump/user_auth.json")
        .expect("can't read user_auth.json\nmake sure the file exists in ./private-dump");
    let cred_obj = json::parse(&cred_file).unwrap();

    let user = UserAuth {
        account: cred_obj["account"].to_string(),
        password : cred_obj["password"].to_string()
    };

    let response: TokenChunk = client.post(host)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&user)
        .send()
        .await?
        .json()
        .await?;

    // response should be the token
    // INFO: debug
    // println!("{:?}", response);
    // INFO: debug
    // println!("{}", token);

    let token = response.result.token.clone();
    Ok(token)
}

// TODO: work
pub async fn markerss() -> Result<(), reqwest::Error>{
    let host = fs::read_to_string("private-dump/pointss-host").expect("can't find pointss-host file containing the auth url\nmake sure the file exists in ./private-dump");
    let client = reqwest::Client::new();
    let token_val = auth().await.expect("authentication failed");
    let response: MarkerChunk = client.get(host)
        .header(TOKEN, token_val)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json()
        .await?;

    for item in response.result {
        println!("id: {:?} name: {:?} code: {:?}", item.id, item.name, item.code);
    }
    // println!("{:?}", response);
    Ok(())
}

#[cfg(test)]
mod tests {
}
