use std::io::{BufWriter, StdoutLock, Write};

use colored::{ColoredString, Colorize};
use terminal_link::Link;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct License {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Phonetic {
    text: Option<String>,
    audio: String,
    source_url: Option<String>,
    license: Option<License>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Definition {
    definition: String,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
    example: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Meaning {
    part_of_speech: String,
    definitions: Vec<Definition>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WordEntry {
    word: String,
    phonetic: Option<String>,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
    license: Option<License>,
    source_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorMessage {
    title: String,
    message: String,
    resolution: String,
}

fn pretty_print(handle: &mut BufWriter<StdoutLock<'static>>, spaces: u32, name: &str, value: ColoredString) {
    let space_string: String = " ".repeat(spaces as usize);
    let pretty_name = name.yellow();
    writeln!(handle, "{}{}{}", space_string, pretty_name, value).unwrap();
}

fn get_pretty_print<'a>(handle: &'a mut BufWriter<StdoutLock<'static>>) -> impl FnMut(u32, &'a str, ColoredString) + 'a{
    |spaces, name, value| pretty_print(handle, spaces, name, value)
}

fn pretty_example(example: String, word: &str) -> String {
    let pattern = format!(r"\b{}\b", regex::escape(word));
    let re = regex::Regex::new(&pattern).unwrap();
    re.replace_all(&example, format!("{}", word.red())).to_string()
}

fn format_license(license: &'_ License) -> Link<'_> {
    Link::new(license.name.as_str(), license.url.as_str())
}

fn print_word_entry(entry: WordEntry) {
    let stdout = std::io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    let mut pretty_print = get_pretty_print(&mut handle);
    let word = entry.word;
    pretty_print(0, "Word: ", word.clone().blue().bold());
    if let Some(phonetic) = entry.phonetic {
        pretty_print(0, "Phonetic: ", phonetic.red());
    }
    if !&entry.phonetics.is_empty() {
        pretty_print(0, "Phonetics:", "".to_string().white());
    }
    for phonetic in entry.phonetics {
        if let Some(text) = phonetic.text {
            pretty_print(2, "Text: ", text.red());
        }
        if !phonetic.audio.is_empty() {
            pretty_print(2, "Audio: ", phonetic.audio.green());
        }
        if let Some(source_url) = phonetic.source_url {
            pretty_print(2, "Source URL: ", source_url.green());
        }
        if let Some(license) = phonetic.license {
            pretty_print(2, "License: ", format_license(&license).to_string().green());
        }
        pretty_print(0, "", "".to_string().white());
    }

    pretty_print(0, "Meanings:", "".to_string().white());
    for meaning in entry.meanings {
        pretty_print(2, "Part of Speech: ", meaning.part_of_speech.magenta());
        for definition in meaning.definitions {
            pretty_print(4, "Definition: ", definition.definition.blue());
            if let Some(example) = definition.example {
                let pretty_example = pretty_example(example, &word);
                pretty_print(4, "Example: ", pretty_example.cyan());
            }
            if !definition.synonyms.is_empty() {
                pretty_print(4, "Synonyms: ", definition.synonyms.join(", ").cyan());
            }
            if !definition.antonyms.is_empty() {
                pretty_print(4, "Antonyms: ", definition.antonyms.join(", ").cyan());
            }
            pretty_print(0, "", "".to_string().white());
        }
    }

    if let Some(license) = entry.license {
        pretty_print(0, "License: ", format_license(&license).to_string().green());
    }

    if !entry.source_urls.is_empty() {
        pretty_print(0, "Source URLs:", "".to_string().white());
        for url in entry.source_urls {
            pretty_print(2, "", url.green());
        }
    }

    std::mem::drop(pretty_print);
    handle.flush().unwrap();
}

fn print_word_entries(word_entries: Vec<WordEntry>) {
    for entry in word_entries {
        print_word_entry(entry)
    }
}

fn print_error_message(error_message: ErrorMessage) {
    let stdout = std::io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    let mut pretty_print = get_pretty_print(&mut handle);
    pretty_print(0, "Title: ", error_message.title.red());
    pretty_print(0, "Message: ", error_message.message.magenta());
    pretty_print(0, "Resolution: ", error_message.resolution.green());
    std::mem::drop(pretty_print);
    handle.flush().unwrap();
}

pub fn process(body: String) {
    let result: Result<Vec<WordEntry>, _> = serde_json::from_str(&body);
    match result {
        Ok(word_entries) => print_word_entries(word_entries),
        Err(_) => {
            let error_message: Result<ErrorMessage, _> = serde_json::from_str(&body);
            match error_message {
                Ok(error_message) => print_error_message(error_message),
                Err(_) => { dbg!(body); },
            }
        }
    }
}
