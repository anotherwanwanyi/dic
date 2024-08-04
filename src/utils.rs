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

fn format_license(license: &'_ License) -> Link<'_> {
    Link::new(license.name.as_str(), license.url.as_str())
}

fn print_word_entry(entry: WordEntry) {
    println!("Word: {}", entry.word);
    if let Some(phonetic) = &entry.phonetic {
        println!("Phonetic: {}", phonetic);
    }
    if !&entry.phonetics.is_empty() {
        println!("Phonetics:");
    }
    for phonetic in &entry.phonetics {
        println!("  Text: {}", phonetic.text);
        if !phonetic.audio.is_empty() {
            println!("  Audio: {}", phonetic.audio);
        }
        if let Some(source_url) = &phonetic.source_url {
            println!("  Source URL: {}", source_url);
        }
        if let Some(license) = &phonetic.license {
            println!("  License: {}", format_license(license));
        }
        println!();
    }

    println!("Meanings:");
    for meaning in &entry.meanings {
        println!("  Part of Speech: {}", meaning.part_of_speech);
        for definition in &meaning.definitions {
            println!("    Definition: {}", definition.definition);
            if let Some(example) = &definition.example {
                println!("    Example: {}", example);
            }
            if !definition.synonyms.is_empty() {
                println!("    Synonyms: {}", definition.synonyms.join(", "));
            }
            if !definition.antonyms.is_empty() {
                println!("    Antonyms: {}", definition.antonyms.join(", "));
            }
            println!();
        }
    }

    if let Some(license) = &entry.license {
        println!("License: {}", format_license(license));
    }

    if !entry.source_urls.is_empty() {
        println!("Source URLs:");
        for url in &entry.source_urls {
            println!("  {}", url);
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
