use std::{fs::File, io::{Read, BufReader, BufRead}};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let mut words = Vec::new();

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            words.push(word.to_owned());
        }
    }

    println!("{:?}", words);
}
