pub fn get_words(lines: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut buffer: Vec<String> = vec![];

    if !lines.is_empty() {
        for word in lines.split_whitespace() {
            buffer.push(word.to_string().replace("**", " ** "));
        }
    
        for word in buffer[0].split_whitespace() {
            result.push(word.to_string())
        }
    }

    result
}