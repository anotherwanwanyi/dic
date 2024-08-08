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

fn pretty_print(spaces: u32, name: &str, value: ColoredString) {
    let space_string: String = " ".repeat(spaces as usize);
    let pretty_name = name.yellow();
    println!("{}{}{}", space_string, pretty_name, value);
}

fn format_license(license: &'_ License) -> Link<'_> {
    Link::new(license.name.as_str(), license.url.as_str())
}

fn print_word_entry(entry: WordEntry) {
    let word = entry.word;
    pretty_print(0, "Word: ", word.clone().blue().bold());
    if let Some(phonetic) = entry.phonetic {
        pretty_print(0, "Phonetic: ", phonetic.red());
    }
    if !&entry.phonetics.is_empty() {
        pretty_print(0, "Phonetics:", "".to_string().white());
    }
    for phonetic in entry.phonetics {
        pretty_print(2, "Text: ", phonetic.text.red());
        if !phonetic.audio.is_empty() {
            pretty_print(2, "Audio: ", phonetic.audio.green());
        }
        if let Some(source_url) = phonetic.source_url {
            pretty_print(2, "Source URL: ", source_url.green());
        }
        if let Some(license) = phonetic.license {
            pretty_print(2, "License: ", format_license(&license).to_string().green());
        }
        println!();
    }

    pretty_print(0, "Meanings:", "".to_string().white());
    for meaning in entry.meanings {
        pretty_print(2, "Part of Speech: ", meaning.part_of_speech.magenta());
        for definition in meaning.definitions {
            pretty_print(4, "Definition: ", definition.definition.cyan());
            if let Some(example) = definition.example {
                pretty_print(4, "Example: ", example.cyan());
            }
            if !definition.synonyms.is_empty() {
                pretty_print(4, "Synonyms: ", definition.synonyms.join(", ").cyan());
            }
            if !definition.antonyms.is_empty() {
                pretty_print(4, "Antonyms: ", definition.antonyms.join(", ").cyan());
            }
            println!();
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
