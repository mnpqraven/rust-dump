use apiutils::{auth, markerss};
// use apiutils::auth;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, long_about = None)]
#[clap(about = "api data fetcher and eventually data manipulation tool")]
struct Args {
    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,

    /// API endpoints, currently 'auth', 'nakedauth' and 'markerss' are available
    #[clap(short, long, value_parser)]
    prop: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // TODO: clear token writer (on user demand only ?)
    // WARNING: lower security, run this locally + isolated only

    // INFO: CLI with Clap
    let args = Args::parse();

    // TODO: enumerate ?
    println!("{}", args.count);

    if args.prop == "markerss" {
        markerss()
            .await
            .expect("error getting markerss, check lib.rs");
    }

    // TODO: cleaner
    if args.prop == "auth" {
        auth().await.expect("authentication error");
        println!("authentication completed, generated token");
    }
    if args.prop == "nakedauth" {
        let naked_token = auth().await.expect("authentication error");
        println!("{}", naked_token);
    }

    Ok(())
}
