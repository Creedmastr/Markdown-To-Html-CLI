#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs;

mod contains;
mod get;
mod html;
mod is;
mod startswith;

use get::get_content;
use get::get_words;

use crate::contains::contains_ch;

fn main() {
    let lines = get_content::get_content();
    let mut result = String::new();
    let mut is_html_valid: bool = true;

    for item in &lines {
        let words = get_words::get_words(item.to_owned());
        let words_is_italic = is::is_italic(words.clone());
        let word_is_underligned = is::is_underligned(words.clone());

        if !item.starts_with("//") {
            match item {
                x if x.starts_with("+/ modifier(") && x.ends_with(")") => {
                    if x.contains("is_html_valid=false") || x.contains("is_html_valid = false") {
                        is_html_valid = false
                    };
                }

                x if x.starts_with("# ") => {
                    result.push_str(&startswith::print_smt(item, "<h1>").as_str());
                }

                x if x.starts_with("## ") => {
                    result.push_str(&startswith::print_smt(item, "<h2>").as_str());
                }

                x if x.starts_with("### ") => {
                    result.push_str(&startswith::print_smt(item, "<h3>").as_str());
                }

                x if x.starts_with("#### ") => {
                    result.push_str(&startswith::print_smt(item, "<h4>").as_str());
                }

                x if x.starts_with("* ") || x.starts_with("+ ") || x.starts_with("- ") => {
                    result.push_str(&startswith::print_smt(item, "<li>").as_str());
                }

                x if x.contains("**") => {
                    result.push_str(
                        contains_ch("**", "<strong>", "</strong>", words.clone()).as_str(),
                    );
                }

                x if words_is_italic.0 => {
                    result.push_str(&startswith::print_smt(item, "<i>"));
                }

                x if word_is_underligned.0 => {
                    result.push_str(&startswith::print_smt(item, "<u>"));
                }

                _ => {
                    result.push_str(&item.clone());
                    result.push('\n');
                }
            }
        }
    }

    if is_html_valid == true {
        result = html::to_valid_html(result.clone());
    }

    fs::write("./output.html", result).expect("Couldn't write file");
}
