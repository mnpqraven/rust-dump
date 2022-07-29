use apiutils::{auth, markerss};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let token = auth()
        .await.expect("authentication error");
    println!("main getting token: {}", token);

    markerss().await.expect("error getting markerss, check lib.rs");
    Ok(())
}
