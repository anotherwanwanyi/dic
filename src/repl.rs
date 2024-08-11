use std::borrow::Cow;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus};

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
