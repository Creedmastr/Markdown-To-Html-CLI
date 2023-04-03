#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs;

mod contains;
mod get;
mod html;
mod is;
mod print_smt;

use get::get_content;
use get::get_words;

use crate::contains::contains_ch;

fn main() {
    let lines = get_content::get_content(); // Get the content of the file
    let mut result = String::new(); // Init the 'result' variable
    let mut is_html_valid: bool = true; // One modifier

    for item in &lines {
        let words = get_words::get_words(item.to_owned());
        
        // Just some conditions
        let words_is_italic = is::is_italic(words.clone());
        let word_is_underligned = is::is_underligned(words.clone());

        // Ignore the commentaries
        if !item.starts_with("//") {
            // Checks item
            match item {
                x if x.starts_with("+/ modifier(") && x.ends_with(")") => {
                    if x.contains("is_html_valid=false") || x.contains("is_html_valid = false") {
                        is_html_valid = false
                    };
                }

                x if x.starts_with("# ") => {
                    result.push_str(&print_smt::print_smt(item, "<h1>").as_str());
                }

                x if x.starts_with("## ") => {
                    result.push_str(&print_smt::print_smt(item, "<h2>").as_str());
                }

                x if x.starts_with("### ") => {
                    result.push_str(&print_smt::print_smt(item, "<h3>").as_str());
                }

                x if x.starts_with("#### ") => {
                    result.push_str(&print_smt::print_smt(item, "<h4>").as_str());
                }

                x if x.starts_with("* ") || x.starts_with("+ ") || x.starts_with("- ") => {
                    result.push_str(&print_smt::print_smt(item, "<li>").as_str());
                }

                x if x.contains("**") => {
                    result.push_str(
                        contains_ch("**", "<strong>", "</strong>", words.clone()).as_str(),
                    );
                }

                x if words_is_italic.0 => {
                    result.push_str(&print_smt::print_smt(item, "<i>"));
                }

                x if word_is_underligned.0 => {
                    result.push_str(&print_smt::print_smt(item, "<u>"));
                }

                _ => {
                    result.push_str(&item.clone());
                    result.push('\n');
                }
            }
        }
    }

    // Check
    if is_html_valid == true {
        result = html::to_valid_html(result.clone());
    }

    fs::write("./output.html", result).expect("Couldn't write file");
}
