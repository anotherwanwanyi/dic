use crate::net::search;
use crate::print::process;

pub fn search_word(word: String) {
    let body = search(word).unwrap();
    process(body);
}
