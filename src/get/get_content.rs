use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Get the content of a file
pub fn get_content(file: File) -> Vec<String> {
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.as_ref().unwrap().to_string());
    }

    return lines;
}
