use anyhow::Result;

pub fn search(word: &str) -> Result<String> {
    let url = "https://api.dictionaryapi.dev/api/v2/entries/en/".to_owned() + word;
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}
