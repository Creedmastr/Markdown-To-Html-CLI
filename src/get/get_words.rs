// Get all the words of a line
pub fn get_words(lines: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut buffer: Vec<String> = vec![];
    let mut index = 0;

    if !lines.is_empty() {
        for word in lines.split_whitespace() {
            buffer.push(word.to_string().replace("**", " ** "));
        }
        for _ in 0..buffer.len() {
            for word in buffer[index].split_whitespace() {
                result.push(word.to_string())
            }

            if !index >= buffer.len() {
                index += 1
            };
        }
    }

    result
}
