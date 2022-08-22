use lookup::lookup;
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
    for line in lookup(&args.host) {
        println!("{}", line);
    }
}
