use cli::{Cli, Commands, GlossaryCommands};
use net::search;
use utils::process;
use repl::DicPrompt;
use clap::Parser;
use colored::Colorize;
use reedline::{DefaultHinter, DefaultValidator, ExampleHighlighter, FileBackedHistory, Reedline, Signal};
use reedline::{default_emacs_keybindings, ColumnarMenu, DefaultCompleter, Emacs, KeyCode, KeyModifiers, ReedlineEvent, ReedlineMenu, MenuBuilder};
use nu_ansi_term::{Color, Style};

mod net;
mod cli;
mod utils;
mod repl;

fn repl() {
    let left_prompt = format!("{}", ">> ".blue().bold());
    let prompt = DicPrompt::new(left_prompt, "".into());
    let file = "/tmp/dic/history.txt".to_string();
    let history = Box::new(
        FileBackedHistory::with_file(100, file.into())
            .expect("Error configuring history with file"),
    );
    let commands = vec![
        "help".into(),
        "search".into(),
        "history".into(),
        "glossary".into(),
    ];
    let mut highlighter = ExampleHighlighter::new(commands.clone());
    highlighter.change_colors(Color::Magenta, Color::Green, Color::Cyan);
    let validator = Box::new(DefaultValidator);
    let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu").with_marker(""));
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Right,
        ReedlineEvent::HistoryHintWordComplete
    );
    let edit_mode = Box::new(Emacs::new(keybindings));

    let mut line_editor =
        Reedline::create()
            .with_history(history)
            .with_highlighter(Box::new(highlighter))
            .with_hinter(Box::new(DefaultHinter::default().with_style(Style::new().italic().fg(Color::LightGray))))
            .with_validator(validator)
            .with_completer(completer)
            .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
            .with_edit_mode(edit_mode);

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

fn search_word(word: String) {
    let body = search(word).unwrap();
    process(body);
}

fn parse_command(cli: Cli) {
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

fn main() {
    let cli = Cli::parse();

    parse_command(cli);
}
