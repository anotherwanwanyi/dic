use cli::Cli;
use net::search;
use utils::process;
use clap::Parser;

mod net;
mod cli;
mod utils;

fn repl() {
    print!("TBC");
    todo!()
}

fn run(word: &str) {
    let value = search(word).unwrap();
    process(value);
}

fn main() {
    let cli = Cli::parse();

    dbg!("word: {:?}", cli.word.as_deref());

    match cli.word.as_deref() {
        Some(word) => run(word),
        None => repl(),
    }
}
