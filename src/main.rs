mod get_content;

fn find_word_line(word: &str, lines: &Vec<String>) -> (String, u32) {
    let mut index: u32 = 0;
    let mut result: String = "".to_string(); 
    for item in lines {
        if item.contains(word) {
            result.push_str(item);
            break;
            
        }
        index += 1;
    };

    (result, index)
}

fn main() {
    let content = get_content::get_content();
    let words = content.0.clone();
    let mut lines = content.1.clone();

    let mut result = String::new();
    
    for item in words {
        let item_word_line = find_word_line(&item, &lines);

        match item {
            x if x == String::from("#") => {
                if item_word_line.1 > 0 && item_word_line.1 < lines.len().try_into().unwrap() {
                    lines.remove(item_word_line.1 as usize - 1);
                }
                result.push_str("<h1> ");
                result.push_str(&item_word_line.0.clone().as_str().replace("# ", "").as_ref());
                result.push_str(" </h1> ");
            }

            _ => {
                result.push_str(&item);
                if item_word_line.1 > 0 && item_word_line.1 < lines.len().try_into().unwrap() {
                    println!("{}", lines[item_word_line.1 as usize - 1]);
                    lines.remove(item_word_line.1 as usize - 1);
                }
            }
        }
    }

    println!("{:#?}", result);
}
