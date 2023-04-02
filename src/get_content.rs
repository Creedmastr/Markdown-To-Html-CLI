use std::{io::{BufReader, BufRead}, fs::File};

pub fn get_content() -> (Vec<String>, Vec<String>) {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let mut words = Vec::new();
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.as_ref().unwrap().to_string());
        for word in line.unwrap().split_whitespace() {
            words.push(word.to_owned());
        }
    }

    return (words, lines);
}