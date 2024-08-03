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

pub fn process(body: String) {
    let word_entries: Vec<WordEntry> = serde_json::from_str(body.as_str()).unwrap();
    dbg!("{}", word_entries);
}
