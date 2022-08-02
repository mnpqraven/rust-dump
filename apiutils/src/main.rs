use crate::util::auth::auth;
use apiutils::{groups, markerss, permission_patch, wipe_token};
use clap::Parser;
mod util;
mod obj;

#[derive(clap::Parser)]
#[clap(author, version, long_about = None)]
struct Args {
    /// Number of maximum items being displayed
    #[clap(short, long, value_parser, default_value_t = 25)]
    limit: u16,

    /// id of the item you're working with
    #[clap(long, value_parser)]
    id: Option<String>,

    /// id of the item you're working with
    #[clap(long, value_parser)]
    id2: Option<String>,

    /// API endpoints
    #[clap(short, long, value_enum)]
    endpoint: Endpoint,

    // TODO: option to save as csv
    #[clap(subcommand)]
    subcommand: Option<SubCommand>,
}

#[derive(clap::ValueEnum, Clone)]
enum Endpoint {
    Auth,
    PermissionPatch,
    Markerss,
    Groups,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    Show,
    CSV,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // TODO: clear token writer (on user demand only ?)
    // WARNING: lower security, run this locally + isolated only

    // INFO: CLI with Clap
    let args = Args::parse();
    // TODO: save to csv
    match args.subcommand {
        Some(SubCommand::CSV) => todo!(),
        _ => {}
    }

    match args.endpoint {
        Endpoint::Auth => match args.subcommand {
            Some(SubCommand::Show) => {
                let naked_token = auth().await.expect("authentication error");
                println!("{}", naked_token);
            }
            _ => {
                auth().await.expect("authentication error");
                println!("authentication completed, generated token");
            }
        },
        Endpoint::PermissionPatch => {
            permission_patch(args.id.unwrap())
                .await
                .expect("error getting markerss, check lib.rs");
        }
        Endpoint::Markerss => {
            markerss(args.limit)
                .await
                .expect("error getting markerss, check lib.rs");
        }
        Endpoint::Groups => {
            groups(args.limit)
                .await
                .expect("error getting groups, check lib.rs");
        }
    }
    wipe_token().expect("can't delete token file");

    Ok(())
}
