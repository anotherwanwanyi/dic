use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use reedline::Signal;

use crate::net::search;
use crate::utils::process;
use crate::repl::{create_line_editor, DicPrompt};

#[derive(Parser, Debug)]
#[command(name = "dic")]
#[command(version = "0.1.0")]
#[command(about = "A REPL CLI Dictionary", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    History,
    Search(WordArg),
    #[command(subcommand)]
    Glossary(GlossaryCommands),
}

#[derive(Subcommand, Debug)]
pub enum GlossaryCommands {
    Add(WordArg),
    Del(WordArg)
}

#[derive(Args, Debug)]
pub struct WordArg {
    pub word: String,
}

fn search_word(word: String) {
    let body = search(word).unwrap();
    process(body);
}

pub fn parse_command(cli: Cli) {
    match cli.command {
        Some(command) => {
            match command {
                Commands::History => println!("History"),
                Commands::Search(arg) => search_word(arg.word),
                Commands::Glossary(subcommand) => {
                    match subcommand {
                        GlossaryCommands::Add(arg) => println!("Add: {}", arg.word),
                        GlossaryCommands::Del(arg) => println!("Del: {}", arg.word),
                    }
                }
            }
        }
        None => repl(),
    }
}

fn repl() {
    let left_prompt = format!("{}", ">> ".blue().bold());
    let prompt = DicPrompt::new(left_prompt, "".into());
    let commands = vec![
        "help".into(),
        "search".into(),
        "history".into(),
        "glossary".into(),
    ];

    let mut line_editor = create_line_editor(commands.clone());

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                let line = buffer.trim();
                if line.is_empty() {
                    continue;
                }
                let mut args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();
                match &args[..] {
                    [ word ] if !commands.contains(word) => search_word(word.to_string()),
                    _ => {
                        args.insert(0, "dic".to_owned());
                        let maybe_cli = Cli::try_parse_from(args);
                        match maybe_cli {
                            Ok(cli) => parse_command(cli),
                            Err(err) => err.print().expect("Error writing Error"),
                        }
                    }
                }
            }
            Signal::CtrlD => {
                println!("{}", "CTRL-D".green());
                break;
            }
            Signal::CtrlC => {
                println!("{}", "CTRL-C".green());
                continue;
            }
        }
    }
}
