use cli::Cli;
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

fn run_command(command: String) {
    let words: Vec<&str> = command.split_whitespace().collect();

    match *words.as_slice() {
        ["glossary", "add", word] => {
            todo!()
        }
        ["glossary", "del", word] => {
            todo!()
        }
        ["help", word] => {
            match word {
                "glossary" => println!("{}", word),
                "history" => println!("{}", word),
                "search" => println!("{}", word),
                "help" => println!("{}", word),
                _ => println!("{} not supported!", word),
            }
        }
        ["help" ] => {
            todo!()
        }
        ["history" ] => {
            todo!()
        }
        ["search", word] | [ word ] => run(word),
        _ => {
            todo!()
        }
    }
}

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
                run_command(buffer);
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
