#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs;
use std::fs::File;

pub mod contains;
pub mod get;
pub mod html;
pub mod is;
pub mod print_smt;

pub use get::get_content;
pub use get::get_words;

pub use crate::contains::contains_ch;

pub fn load(file: File, mut is_html_valid: bool, write_file: bool) -> String {
    // Get the content of the file
    let lines = get_content::get_content(file);
    let mut result = String::new(); // Init the 'result' variable

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

    if write_file == true {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            x if x >= 3 => {
                if args[2].ends_with(".html") {
                    fs::write(&args[2], &result).expect("Couldn't write file");
                } else {
                    let mut arg1 = args[2].clone();
                    arg1.push_str(".html");
                    fs::write(arg1, &result).expect("Couldn't write file");
                }
            }
    
            _ => {
                fs::write("./output.html", &result).expect("Couldn't write file");
            }
        }
    }

    result.clone()
}
