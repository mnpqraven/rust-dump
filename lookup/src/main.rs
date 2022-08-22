use lookup::lookupv4;
use clap::Parser;


/// ip lookup tool
#[derive(Parser)]
#[clap(about)]
struct Args {
    /// Domain
    #[clap(value_parser)]
    host: String
}

fn main() {
    let args = Args::parse();
    for line in lookupv4(&args.host) {
        println!("{}", line);
    }
}
