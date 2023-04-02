use std::fs;

mod get;

use get::get_content;
use get::get_words;

fn main() {
    let lines = get_content::get_content();
    let mut result = String::new();
    
    for item in &lines {
        match item {
            x if x.starts_with("#") => {
                result.push_str("<h1> ");
                result.push_str(&item.clone().as_str().replace("# ", "").as_ref());
                result.push_str(" </h1> ");
                result.push('\n');
            },

            x if x.contains("**") => {
                let words = get_words::get_words(item.to_owned());
                let mut buffer: u8 = 0;

                for word in words {
                    if word.contains( "**") && buffer == 0 {
                        buffer = 1;
                        result.push_str("<strong>")
                    } else if word.contains( "**") && buffer == 1 {
                        buffer = 0;
                        result.push_str("</strong>")
                    } else {
                        result.push_str(&word);
                    }
                }
            },

            _ => {
                result.push_str(&item.clone());
                result.push('\n');
            }      
        }
        
    }

    println!("{:#?}", result);
    fs::write("./output.html", result).expect("Couldn't write file");
}
