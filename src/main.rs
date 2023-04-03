#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs;

mod contains;
mod get;
mod startswith;
mod is;
mod html;

use get::get_content;
use get::get_words;

use crate::contains::contains_ch;

fn main() {
    let lines = get_content::get_content();
    let mut result = String::new();

    for item in &lines {
        let words = get_words::get_words(item.to_owned());
        let words_is_italic = is::is_italic(words.clone());
        let word_is_underligned = is::is_underligned(words.clone());

        match item {
            x if x.starts_with("# ") => {
                result.push_str(startswith::smt_print(item, " <h1> ").as_str());
            }

            x if x.starts_with("## ") => {
                result.push_str(startswith::smt_print(item, " <h2> ").as_str());
            }

            x if x.starts_with("### ") => {
                result.push_str(startswith::smt_print(item, " <h3> ").as_str());
            }

            x if x.starts_with("#### ") => {
                result.push_str(startswith::smt_print(item, " <h4> ").as_str());
            }

            x if x.contains("**") => {
                result.push_str(contains_ch("**", "<strong>", "</strong>", words.clone()).as_str())
            }

            x if words_is_italic.0 => {
                let mut li = String::new();

                li.push_str("<li>");
                li.push_str(words_is_italic.1.replace("*", "").as_str());
                li.push_str("</li>");
                li.push('\n');

                result.push_str(li.as_str());
            }

            x if word_is_underligned.0 => {
                let mut u = String::new();

                u.push_str("<u>");
                u.push_str(word_is_underligned.1.replace("__", "").as_str());
                u.push_str("</u>");
                u.push('\n');

                result.push_str(u.as_str());
            }

            _ => {
                result.push_str(&item.clone());
                result.push('\n');
            }
        }
    }

    // TODO: Separate it into a different file, and make the output.html file a normal html file (with a template etc.)
    result = html::to_valid_html(result.clone());

    fs::write("./output.html", result).expect("Couldn't write file");
}
