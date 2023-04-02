use std::{io::{BufReader, BufRead}, fs::File};

pub fn get_content() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.as_ref().unwrap().to_string());
    }

    return lines;
}