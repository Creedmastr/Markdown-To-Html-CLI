#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs;

mod contains;
mod get;
mod startswith;

use get::get_content;
use get::get_words;

use crate::contains::contains_ch;

fn is_italic(vec: Vec<String>) -> (bool, String) {
    let mut index: u32 = 0;
    for item in vec {
        if item.starts_with("*") && item.ends_with("*") && !item.ends_with("**") && !item.starts_with("**") {
            return (true, item);
        }

        index += 1;
    }

    return (false, "None".to_string());
}

fn main() {
    let lines = get_content::get_content();
    let mut result = String::new();
    let mut li = String::new();

    for item in &lines {
        let words = get_words::get_words(item.to_owned());
        let words_is_italic = is_italic(words.clone());
        println!("{:#?}", words);

        match item {
            x if x.starts_with("#") => {
                result.push_str(startswith::smt_print(item, " <h1> ").as_str());
            }

            x if x.starts_with("##") => {
                result.push_str(startswith::smt_print(item, " <h2> ").as_str());
            }

            x if x.starts_with("###") => {
                result.push_str(startswith::smt_print(item, " <h3> ").as_str());
            }

            x if x.starts_with("####") => {
                result.push_str(startswith::smt_print(item, " <h4> ").as_str());
            }

            x if x.contains("**") => {
                result.push_str(contains_ch("**", "<strong>", "</strong>", words.clone()).as_str())
            }

            x if words_is_italic.0 => {
                li.push_str("<li>");
                li.push_str(words_is_italic.1.replace("*", "").as_str());
                li.push_str("</li>");
                li.push('\n');
                result.push_str(li.as_str());
            }

            _ => {
                result.push_str(&item.clone());
                result.push('\n');
            }
        }
    }

    println!("{:#?}", result);
    fs::write("./output.html", result).expect("Couldn't write file");
}
