pub fn get_words(lines: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for word in lines.split_whitespace() {
        result.push(word.to_string().replace("**", " ** "));
    }

    result
}