use crate::obj::token::TokenCollection;
use crate::obj::system::UserAuth;

use std::fs;
use reqwest::header::*;

use json;
use tokio::fs as tokiofs;

use super::file::get_host;

pub async fn auth() -> Result<String, reqwest::Error> {
    let mut host: String = get_host(true).unwrap();
    host.push_str("auth/login");
    let client = reqwest::Client::new();

    let cred_file = fs::read_to_string("private-dump/user_auth.json")
        .expect("can't read user_auth.json\nmake sure the file exists in ./private-dump");
    let cred_obj = json::parse(&cred_file).unwrap();

    let user = UserAuth {
        account: cred_obj["account"].to_string(),
        password: cred_obj["password"].to_string(),
    };

    let response: TokenCollection = client
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
