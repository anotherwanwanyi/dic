use anyhow::Result;

pub fn search(word: String) -> Result<String> {
    let url = format!("{}{}", "https://api.dictionaryapi.dev/api/v2/entries/en/", word);
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}
