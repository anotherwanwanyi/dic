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
    text: String,
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

enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

fn colored_value(value: String, color: Colors) -> String {
   match color {
        Colors::Black => stylish::ansi::format!("{:(fg=black)}", value),
        Colors::Red => stylish::ansi::format!("{:(fg=red)}", value),
        Colors::Green => stylish::ansi::format!("{:(fg=green)}", value),
        Colors::Yellow => stylish::ansi::format!("{:(fg=yellow)}", value),
        Colors::Blue => stylish::ansi::format!("{:(fg=blue)}", value),
        Colors::Magenta => stylish::ansi::format!("{:(fg=magenta)}", value),
        Colors::Cyan => stylish::ansi::format!("{:(fg=cyan)}", value),
        Colors::White => stylish::ansi::format!("{:(fg=white)}", value),
        Colors::Default => stylish::ansi::format!("{}", value),
        // _ => "colored_value unimplemented".to_string()
    } 
}

fn pretty_print(spaces: u32, name: &str, value: String, color: Colors) {
    let space_str: String = " ".repeat(spaces as usize);

    let pretty_name = stylish::ansi::format!("{}{:(fg=yellow)}", space_str, name);

    let pretty_value = colored_value(value, color);

    println!("{}{}", pretty_name, pretty_value);
}

fn format_license(license: &'_ License) -> Link<'_> {
    Link::new(license.name.as_str(), license.url.as_str())
}

fn print_word_entry(entry: WordEntry) {
    pretty_print(0, "Word: ", entry.word, Colors::Blue);
    if let Some(phonetic) = entry.phonetic {
        pretty_print(0, "Phonetic: ", phonetic, Colors::Red);
    }
    if !&entry.phonetics.is_empty() {
        pretty_print(0, "Phonetics:", "".to_string(), Colors::Default);
    }
    for phonetic in entry.phonetics {
        pretty_print(2, "Text: ", phonetic.text, Colors::Red);
        if !phonetic.audio.is_empty() {
            pretty_print(2, "Audio: ", phonetic.audio, Colors::Green);
        }
        if let Some(source_url) = phonetic.source_url {
            pretty_print(2, "Source URL: ", source_url, Colors::Green);
        }
        if let Some(license) = phonetic.license {
            pretty_print(2, "License: ", format_license(&license).to_string(), Colors::Green);
        }
        println!();
    }

    pretty_print(0, "Meanings:", "".to_string(), Colors::Default);
    for meaning in entry.meanings {
        pretty_print(2, "Part of Speech: ", meaning.part_of_speech, Colors::Magenta);
        for definition in meaning.definitions {
            pretty_print(4, "Definition: ", definition.definition, Colors::Cyan);
            if let Some(example) = definition.example {
                pretty_print(4, "Example: ", example, Colors::Cyan);
            }
            if !definition.synonyms.is_empty() {
                pretty_print(4, "Synonyms: ", definition.synonyms.join(", "), Colors::Cyan);
            }
            if !definition.antonyms.is_empty() {
                pretty_print(4, "Antonyms: ", definition.antonyms.join(", "), Colors::Cyan);
            }
            println!();
        }
    }

    if let Some(license) = entry.license {
        pretty_print(0, "License: ", format_license(&license).to_string(), Colors::Green);
    }

    if !entry.source_urls.is_empty() {
        pretty_print(0, "Source URLs:", "".to_string(), Colors::Default);
        for url in entry.source_urls {
            pretty_print(2, "", url, Colors::Green);
        }
    }

    println!();
}

fn print_word_entries(word_entries: Vec<WordEntry>) {
    for entry in word_entries {
        print_word_entry(entry)
    }
}

pub fn process(body: String) {
    let word_entries: Vec<WordEntry> = serde_json::from_str(body.as_str()).unwrap();
    print_word_entries(word_entries)
}
