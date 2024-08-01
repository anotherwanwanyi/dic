use cli::Cli;
use net::search;
use clap::Parser;

mod net;
mod cli;

fn repl() {
    todo!()
}

fn run(word: &str) {
    search(word).unwrap();
    todo!()
}

fn main() {
    let cli = Cli::parse();

    println!("word: {:?}", cli.word.as_deref());

    match cli.word.as_deref() {
        Some(word) => run(word),
        None => repl(),
    }
}
