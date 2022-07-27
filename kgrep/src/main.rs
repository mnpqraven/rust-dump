use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    pattern: String
}

fn main() {
    let args = Cli::parse();
    let filedata = std::fs::read_to_string(&args.path);
    let content = match filedata {
        Ok(content) => content,
        Err(error) => panic!("monka error {}", error)
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
