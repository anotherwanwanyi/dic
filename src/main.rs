use cli::{Cli, parse_command};
use clap::Parser;

mod net;
mod cli;
mod print;
mod utils;
mod repl;

fn main() {
    let cli = Cli::parse();

    parse_command(cli);
}
