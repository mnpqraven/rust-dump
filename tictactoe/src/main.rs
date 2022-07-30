use tictactoe::draw_board;
use std::io;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Size of the board
    /// Default: 3
    /// NOTE: unused
    #[clap(value_parser)]
    size: Option<usize>,

    /// Number of tics in line to win
    /// Default: 4
    #[clap(value_parser, default_value_t = 4)]
    needed_to_win: usize,

    /// +1 mode means when one end of a line is connected to the other player's
    /// unit, you need to have an extra unit in the line to win (e.g: if it's
    /// usually 4 in line to win and one end has the other player's unit, then
    /// you need 5 in line to win)
    /// Enable +1 mode ?
    /// Default: false
    #[clap(default_value_t = false)]
    plusone: bool
}

fn main() -> Result<(), &'static str>{
    let cli = Cli::parse();
    let mut input = String::new();

    // INFO: runtime
    println!("Enter board size");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read your input");
    // WARN: needs &str for parse()
    match input.trim().parse::<usize>() {
        Ok(n) => draw_board(n).expect("can't draw the board"),
        Err(_) => return Err("yo bruh u high")
    }

    // TODO: gameplay loop
    // correct number input: proceed
    // incorrect number input: ignore + retry
    // win/lose check
    spot_input(1, 1);
    result_check();
    Ok(())
}

fn spot_input(x: usize, y: usize) -> Result<(), &'static str>{
    todo!();
    // recursion until SIGOUT or wins
    spot_input(x, y);
}
fn result_check() {
    unimplemented!()
}
