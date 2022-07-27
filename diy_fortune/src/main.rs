use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let res = reqwest::get("https://fortuneapi.herokuapp.com/")
        .await?
        .text()
        .await?;
    println!("{:#?}", res);
    Ok(())
}
