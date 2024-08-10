use cli::Cli;
use net::search;
use utils::process;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use colored::Colorize;

mod net;
mod cli;
mod utils;

fn repl() {
    let mut rl = DefaultEditor::new().unwrap();
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let prompt = format!("{}", ">> ".blue().bold());
    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                run(&line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("{}", "CTRL-C".green());
                break
            },
            Err(ReadlineError::Eof) => {
                println!("{}", "CTRL-D".green());
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt").unwrap();
}

fn run(word: &str) {
    let body = search(word).unwrap();
    process(body);
}

fn main() {
    let cli = Cli::parse();

    match cli.word.as_deref() {
        Some(word) => run(word),
        None => repl(),
    }
}
