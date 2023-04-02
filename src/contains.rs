use crate::get::get_words::{get_words, self};

pub fn contains_ch(ch: &str, to_push_first: &str, to_push_second: &str, words: Vec<String>) -> String {
    let mut result = String::new();
    let mut buffer: u8 = 0;

    for mut word in words.clone() {
        if word.contains(ch) && buffer == 0 {
            buffer = 1;
            result.push_str(to_push_first)
        } else if word.contains(ch) && buffer == 1 {
            buffer = 0;
            result.push_str(to_push_second);
            result.push('\n');
        } else {
            result.push_str(&word);
        }
    }

    result
}
