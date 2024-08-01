use clap::Parser;

#[derive(Parser)]
#[command(name = "dic")]
#[command(version = "0.1.0")]
#[command(about = "A REPL CLI Dictionary", long_about = None)]
pub struct Cli {
    pub word: Option<String>,
    // #[arg(short, long, value_name = "WORD")]
    // search: Option<String>,
}

// pub fn cli() {
//     let cli = Cli::parse();

//     println!("word: {:?}", cli.word.as_deref());
//     // println!("search: {:?}", cli.search.as_deref());
// }
