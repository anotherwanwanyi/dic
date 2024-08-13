use std::borrow::Cow;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus, Reedline};
use reedline::{DefaultHinter, DefaultValidator, ExampleHighlighter, FileBackedHistory};
use reedline::{default_emacs_keybindings, ColumnarMenu, DefaultCompleter, Emacs, KeyCode, KeyModifiers, ReedlineEvent, ReedlineMenu, MenuBuilder};
use nu_ansi_term::{Color, Style};

pub struct DicPrompt {
    left: String,
    right: String,
}

impl Prompt for DicPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Borrowed(&self.left)
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed(&self.right)
    }

    fn render_prompt_indicator(&self, edit_mode: PromptEditMode) -> Cow<str> {
        let _ = edit_mode;
        "".into()
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        "".into()
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };
        // NOTE: magic strings, given there is logic on how these compose I am not sure if it
        // is worth extracting in to static constant
        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}

impl DicPrompt {
    pub fn new(left: String, right: String) -> DicPrompt {
        DicPrompt {
            left,
            right,
        }
    }
}

pub fn create_line_editor(commands: Vec<String>) -> Reedline {
    let file = "/tmp/dic/history.txt".to_string();
    let history = Box::new(
        FileBackedHistory::with_file(100, file.into())
            .expect("Error configuring history with file"),
    );
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

    Reedline::create()
        .with_history(history)
        .with_highlighter(Box::new(highlighter))
        .with_hinter(Box::new(DefaultHinter::default().with_style(Style::new().italic().fg(Color::LightGray))))
        .with_validator(validator)
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode)
}

