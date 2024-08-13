use clap::{Args, Parser, Subcommand};

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
    #[command(subcommand)]
    Glossary(GlossaryCommands),
    Search(WordArg)
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
