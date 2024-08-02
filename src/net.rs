use anyhow::Result;
use serde_json::Value;

pub fn search(word: &str) -> Result<Value> {
    let url = "https://api.dictionaryapi.dev/api/v2/entries/en/".to_owned() + word;
    let body = reqwest::blocking::get(url)?.text()?;
    dbg!(&body);
    let value = serde_json::from_str(body.as_str())?;
    Ok(value)
}
